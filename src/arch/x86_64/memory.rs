use core::mem::transmute;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::VirtAddr;
use x86_64::structures::paging::page_table::PageTable;
use x86_64::structures::paging::mapper::OffsetPageTable;

lazy_static! {
    static ref PAGE_TABLE: Mutex<OffsetPageTable<'static>> = unsafe {
        let table: &'static mut PageTable = transmute(x86_64::registers::control::Cr3::read().0);
        Mutex::new(OffsetPageTable::new(table, VirtAddr::zero()))
    };
}

pub fn initialize() {
    super::unique::gdt::load();
    super::unique::idt::load();
}