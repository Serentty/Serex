pub fn initialize() {
    super::unique::gdt::load();
    super::unique::idt::load();
}