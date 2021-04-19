use core::ptr::{read_volatile, write_volatile};

const MAILBOX_BASE: usize = 0x3f00b880 as _;
const STATUS_OFFSET: usize = 0x18;
const WRITE_OFFSET: usize = 0x20;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Channel {
    PowerManagement = 0,
    Framebuffer = 1,
    VirtualUart = 2,
    Vchiq = 3,
    Leds = 4,
    Buttons = 5,
    TouchScreen = 6,
    PropertyTags1 = 8,
    PropertyTags2 = 9
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MailboxError {
    InvalidArgument,
    NotReady,
    WrongChannel
}

fn read(ch: Channel) -> Result<u32, MailboxError> {
    use core::mem::transmute;

    unsafe {
	let mailbox: *mut u32 = transmute(MAILBOX_BASE);
	let status: *mut u32 = transmute(MAILBOX_BASE + STATUS_OFFSET);

	if (read_volatile(status) & 0x40000000) != 0 {
	    return Err(MailboxError::NotReady);
	}
	
	let data = read_volatile(mailbox);
	
	if data & (ch as u32) == ch as u32 {
	    Ok(data & 0xFFFFFFF0)
	} else {
	    Err(MailboxError::WrongChannel)
	}
	
    }
}

fn write(ch: Channel, message: u32) -> Result<(), MailboxError> {
    use core::mem::transmute;
    
    if message & 0xF != 0 { return Err(MailboxError::InvalidArgument); }

    unsafe {
	let mailbox: *mut u32 = transmute(MAILBOX_BASE);
	let status: *mut u32 = transmute(MAILBOX_BASE + STATUS_OFFSET);
	let write: *mut u32 = transmute(MAILBOX_BASE + WRITE_OFFSET);

	if read_volatile(status) & 0x80000000 != 0 {
	    return Err(MailboxError::NotReady);
	}

	write_volatile(write, message | (ch as u32));
    }
    
    Ok(())
}

pub fn arm_to_vc(address: usize) -> usize {
    address + 0x40000000
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
enum PropertyTag {
    Null = 0,
    FbAllocateBuffer = 0x00040001,
    FbReleaseBuffer = 0x00048001,
    FbGetPhysicalDimensions = 0x00040003,
    FbSetPhysicalDimensions = 0x00048003,
    FbGetVirtualDimensions = 0x00040004,
    FbSetVirtualDimensions = 0x00048004,
    FbGetBitsPerPixel = 0x00040005,
    FbSetBitsPerPixel = 0x00048005,
    FbGetBytesPerRow = 0x00040008
}

#[repr(C)]
union ValueBuffer {
    uint32: u32,
    screen_size: ScreenSize,
    framebuffer_allocation_result: FramebufferAllocationResult
}

#[repr(C)]
struct PropertyMessageTag {
    tag: PropertyTag,
    value_buffer: ValueBuffer
}

impl PropertyMessageTag {
    const fn value_buffer_len(&self) -> u32 {
        use PropertyTag::*;
        match self.tag {
            FbAllocateBuffer
                | FbGetPhysicalDimensions
                | FbSetPhysicalDimensions
                | FbGetVirtualDimensions
                | FbSetVirtualDimensions
		=> { 8 },
            FbGetBitsPerPixel
		| FbSetBitsPerPixel
                | FbGetBytesPerRow
                => { 4 },
            FbReleaseBuffer
		=> { 0 },
            Null
                => { 0 }
	}
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
enum PropertyMessageRequestResponseCode {
    Request = 0x00000000,
    ResponseSuccess = 0x80000000,
    ResponseError = 0x80000001
}

#[repr(C)]
#[repr(align(16))]
struct PropertyMessageBufferHeader {
    size: u32,
    request_response_code: PropertyMessageRequestResponseCode
}

fn send_messages(tags: &mut [PropertyMessageTag]) -> PropertyMessageRequestResponseCode {
    use super::mailbox::*;

    let mut buffer_size = 0;

    for tag in tags.iter() {
	buffer_size += tag.value_buffer_len() + 12; // 12 for tag header stuff.
    }

    // Align the buffer size to 16 bytes.
    buffer_size += if (buffer_size % 16) != 0 { 16 - (buffer_size % 16) } else { 0 };

    unsafe {
	use core::mem::*;
	use core::ptr::copy_nonoverlapping as cpno;
	use alloc::vec::Vec;
	
	let header = PropertyMessageBufferHeader {
	    size: buffer_size,
	    request_response_code: PropertyMessageRequestResponseCode::Request,
	};

	let mut buffer: Vec<u8> = Vec::with_capacity(1024);
	let mut offset: isize = 0;
	cpno(transmute(&header), buffer.as_mut_ptr(), size_of_val(&header));
	offset += size_of_val(&header) as isize;
	
	let zero: u32 = 0;
	
	for tag in tags.iter() {
	    let len = tag.value_buffer_len();
	    cpno(transmute(&tag.tag), buffer.as_mut_ptr().offset(offset), size_of_val(&tag.tag));
	    offset += size_of_val(&tag.tag) as isize;
	    cpno(transmute(&len), buffer.as_mut_ptr().offset(offset), size_of_val(&len));
	    offset += size_of_val(&len) as isize;
	    cpno(transmute(&zero), buffer.as_mut_ptr().offset(offset), size_of_val(&zero));
	    offset += size_of_val(&zero) as isize;
	    cpno(transmute(&tag.value_buffer), buffer.as_mut_ptr().offset(offset), len as usize);
	    offset += len as isize;
	}

	cpno(transmute(&zero), buffer.as_mut_ptr().offset(offset), size_of_val(&zero));

	write(Channel::PropertyTags1, buffer.as_ptr() as _);
	read(Channel::PropertyTags1);	
	let response_code: *const PropertyMessageRequestResponseCode = transmute(buffer.as_ptr().offset(4));
	let response_code = *response_code;

	// Copy the response back.
	let mut offset: isize = 0;
	for mut tag in tags {
	    let len = tag.value_buffer_len();
	    offset += 12; // Skip header stuff.
	    cpno(transmute(buffer.as_ptr().offset(offset)), &mut tag.value_buffer, len as usize);
	    offset += len as isize;
	}

	response_code
    }
}

const FRAMEBUFFER_WIDTH: u32 = 640;
const FRAMEBUFFER_HEIGHT: u32 = 480;
const FRAMEBUFFER_DEPTH: u32 = 24;

#[derive(Copy, Clone)]
#[repr(C)]
struct FramebufferLocation {
    address: usize,
    size: u32
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct ScreenSize {
    x: u32,
    y: u32
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct FramebufferAllocationResult {
    base: *mut volatile::Volatile<u8>,
    len: u32
}

#[derive(Debug)]
pub struct FramebufferInformation {
    width: u32,
    height: u32,
    base: *mut volatile::Volatile<u8>,
    len: usize
}

pub fn initialize_framebuffer() -> FramebufferInformation {
    unsafe {
	let mut property_messages = {
	    use PropertyTag::*;
	    [
		PropertyMessageTag {
		    tag: FbSetPhysicalDimensions,
		    value_buffer: ValueBuffer {
			screen_size: ScreenSize { x: FRAMEBUFFER_WIDTH, y: FRAMEBUFFER_HEIGHT }
		    }
		},
		PropertyMessageTag {
		    tag: FbSetVirtualDimensions,
		    value_buffer: ValueBuffer {
			screen_size: ScreenSize { x: FRAMEBUFFER_WIDTH, y: FRAMEBUFFER_HEIGHT }
		    }
		},
		PropertyMessageTag {
		    tag: FbSetBitsPerPixel,
		    value_buffer: ValueBuffer {
			uint32: FRAMEBUFFER_DEPTH
		    }
		},
		PropertyMessageTag {
		    tag: Null,
		    value_buffer: ValueBuffer {
			uint32: 0
		    }
		}
	    ]
	};
	
	crate::println!("{:?}", send_messages(&mut property_messages));
	
	let width = property_messages[0].value_buffer.screen_size.x;
	let height = property_messages[0].value_buffer.screen_size.y;
	
	let mut framebuffer_request = {
	    use PropertyTag::*;
	    [
		PropertyMessageTag {
		    tag: FbAllocateBuffer,
		    value_buffer: ValueBuffer {
			uint32: 16 // This is the buffer's alignment.
		    }
		},
		PropertyMessageTag {
		    tag: Null,
		    value_buffer: ValueBuffer { uint32: 0 }
		}
	    ]
	};
	
	crate::println!("{:?}", send_messages(&mut framebuffer_request));

	let fb = FramebufferInformation {
	    height,
	    width,
	    base: framebuffer_request[0].value_buffer.framebuffer_allocation_result.base.offset(0x40000000),
            len: framebuffer_request[0].value_buffer.framebuffer_allocation_result.len as usize
	};

	for i in 0..1024 {
	    fb.base.offset(i).write(volatile::Volatile::new(0xFF));
	}

	fb
    }
}
