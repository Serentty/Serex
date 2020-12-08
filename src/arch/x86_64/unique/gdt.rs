use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

pub fn load() {
    GLOBAL_DESCRIPTOR_TABLE.0.load();
    unsafe {
        x86_64::instructions::segmentation::set_cs(GLOBAL_DESCRIPTOR_TABLE.1.code_selector);
        x86_64::instructions::tables::load_tss(GLOBAL_DESCRIPTOR_TABLE.1.tss_selector);
    }
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

lazy_static! {
    static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TASK_STATE_SEGMENT));
        let selectors = Selectors { code_selector, tss_selector };
        (gdt, selectors)
    };
}

pub const DOUBLE_FAULT_STACK_INDEX: u16 = 0;
pub const PAGE_FAULT_STACK_INDEX: u16 = 1;
lazy_static! {
    static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 8;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
                stack_end
        };
        tss.interrupt_stack_table[PAGE_FAULT_STACK_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 8;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}