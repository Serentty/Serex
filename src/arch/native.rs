#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::console;
#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::memory;
#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::io;
#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::interrupts;