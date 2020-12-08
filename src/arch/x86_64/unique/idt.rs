use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::println;
use pic::PicInterrupt;

pub mod pic;

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            // CPU exceptions
            idt.divide_error.set_handler_fn(idt_divide_by_zero);
            idt.debug.set_handler_fn(idt_debug);
            idt.non_maskable_interrupt.set_handler_fn(idt_non_maskable_interrupt);
            idt.breakpoint.set_handler_fn(idt_breakpoint);
            idt.overflow.set_handler_fn(idt_overflow);
            idt.bound_range_exceeded.set_handler_fn(idt_bound_range_exceeded);
            idt.invalid_opcode.set_handler_fn(idt_invalid_opcode);
            idt.device_not_available.set_handler_fn(idt_device_not_available);
            idt.double_fault.set_handler_fn(idt_double_fault).set_stack_index(super::gdt::DOUBLE_FAULT_STACK_INDEX);
            idt.invalid_tss.set_handler_fn(idt_invalid_tss);
            idt.segment_not_present.set_handler_fn(idt_segment_not_present);
            idt.stack_segment_fault.set_handler_fn(idt_stack_segment_fault);
            idt.general_protection_fault.set_handler_fn(idt_general_protection_fault);
            idt.page_fault.set_handler_fn(idt_page_fault).set_stack_index(super::gdt::PAGE_FAULT_STACK_INDEX);
            idt.x87_floating_point.set_handler_fn(idt_x87_floating_point);
            idt.alignment_check.set_handler_fn(idt_alignment_check);
            idt.machine_check.set_handler_fn(idt_machine_check);
            idt.simd_floating_point.set_handler_fn(idt_simd_floating_point);
            idt.virtualization.set_handler_fn(idt_virtualization);
            // PIC interrupts
            idt[PicInterrupt::Timer as usize].set_handler_fn(pic::idt_timer);
            idt[PicInterrupt::Keyboard as usize].set_handler_fn(pic::idt_keyboard);
        }
        idt
    };
}

pub fn load() {
    INTERRUPT_DESCRIPTOR_TABLE.load();
}

extern "x86-interrupt" fn idt_divide_by_zero(frame: &mut InterruptStackFrame)  {
    println!("Divide-by-zero error {:#?}", frame);
}

extern "x86-interrupt" fn idt_debug(frame: &mut InterruptStackFrame)  {
    println!("Debug {:#?}", frame);
}

extern "x86-interrupt" fn idt_non_maskable_interrupt(frame: &mut InterruptStackFrame)  {
    println!("Non-maskable interrupt {:#?}", frame);
}

extern "x86-interrupt" fn idt_breakpoint(frame: &mut InterruptStackFrame) {
    println!("Breakpoint {:#?}", frame);
}

extern "x86-interrupt" fn idt_overflow(frame: &mut InterruptStackFrame) {
    println!("Overflow {:#?}", frame);
}

extern "x86-interrupt" fn idt_bound_range_exceeded(frame: &mut InterruptStackFrame) {
    println!("Bound range exceeded {:#?}", frame);
}

extern "x86-interrupt" fn idt_invalid_opcode(frame: &mut InterruptStackFrame) {
    println!("Invalid opcode {:#?}", frame);
}

extern "x86-interrupt" fn idt_device_not_available(frame: &mut InterruptStackFrame) {
    println!("Device not available {:#?}", frame);
}

extern "x86-interrupt" fn idt_double_fault(frame: &mut InterruptStackFrame, _error: u64) -> ! {
    panic!("Double fault {:#?}", frame);
}

extern "x86-interrupt" fn idt_invalid_tss(frame: &mut InterruptStackFrame, _error: u64) {
    println!("Invalid TSS {:#?}", frame);
}

extern "x86-interrupt" fn idt_segment_not_present(frame: &mut InterruptStackFrame, _error: u64) {
    println!("Segment not present {:#?}", frame);
}

extern "x86-interrupt" fn idt_stack_segment_fault(frame: &mut InterruptStackFrame, _error: u64) {
    println!("Stack segment fault {:#?}", frame);
}

extern "x86-interrupt" fn idt_general_protection_fault(frame:  &mut InterruptStackFrame, _error: u64) {
    println!("General protection fault {:#?}", frame);
}

extern "x86-interrupt" fn idt_page_fault(frame: &mut InterruptStackFrame, _error: PageFaultErrorCode) {
    println!("Page fault {:#?}", frame);
}

extern "x86-interrupt" fn idt_x87_floating_point(frame: &mut InterruptStackFrame) {
    println!("x87 floating point {:#?}", frame);
}

extern "x86-interrupt" fn idt_alignment_check(frame:  &mut InterruptStackFrame, _error: u64) {
    println!("Alighnment check {:#?}", frame);
}

extern "x86-interrupt" fn idt_machine_check(frame: &mut InterruptStackFrame) -> ! {
    panic!("Machine check {:#?}", frame);
}

extern "x86-interrupt" fn idt_simd_floating_point(frame: &mut InterruptStackFrame) {
    println!("SIMD floating point {:#?}", frame);
}

extern "x86-interrupt" fn idt_virtualization(frame: &mut InterruptStackFrame) {
    println!("Virtualization {:#?}", frame);
}

extern "x86-interrupt" fn idt_security_exception(frame: &mut InterruptStackFrame, _error: u64) {
    println!("Security exception {:#?}", frame);
}
