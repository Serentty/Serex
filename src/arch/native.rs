#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::console;
#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::memory;
#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::io;
#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::interrupts;

#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::console;
#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::memory;
#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::io;
#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::interrupts;