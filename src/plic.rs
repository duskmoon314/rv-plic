/*!
Platform Level Interrupt Controller
*/

use crate::registers;

/// Priority of interrupt
///
/// BIT = 3 => priority = 0..=7, 0 means never trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority<const BIT: usize>(pub(crate) u32);

impl<const BIT: usize> Priority<BIT> {
    /// (Use in threshold) Any priorities will trigger
    pub fn any() -> Priority<BIT> {
        Priority(0)
    }

    /// Never trigger an interrupt
    pub fn never() -> Priority<BIT> {
        Priority(0)
    }

    /// The lowest priority to trigger an interrupt
    pub fn lowest() -> Priority<BIT> {
        Priority(1)
    }

    /// The highest priority to trigger an interrupt
    pub fn highest() -> Priority<BIT> {
        match BIT {
            32 => Priority(u32::MAX),
            _ => Priority((2 << BIT) - 1),
        }
    }
}

impl<const BIT: usize> From<u32> for Priority<BIT> {
    fn from(priority: u32) -> Self {
        if BIT == 32 {
            return Priority(priority);
        }
        if priority < 2 << BIT {
            Priority(priority)
        } else {
            panic!("invalid priority")
        }
    }
}

impl<const BIT: usize> From<Priority<BIT>> for u32 {
    fn from(priority: Priority<BIT>) -> Self {
        priority.0
    }
}

/// # Platform Level Interrupt Controller
///
/// ## Usage
///
/// Set qemu virt machine uart:
///
/// ```no_run
/// # use rv_plic::*;
/// pub const PLIC_BASE: usize = 0xc00_0000;
/// pub const PLIC_PRIORITY_BIT: usize = 3;
/// type plic = PLIC<PLIC_BASE, PLIC_PRIORITY_BIT>;
/// plic::set_threshold(1, Priority::any());
/// plic::enable(1, 10);
/// plic::set_priority(10, Priority::lowest());
/// ```
pub struct PLIC<const BASE: usize, const BIT: usize>;

impl<const BASE: usize, const BIT: usize> PLIC<BASE, BIT> {
    const REGS: *const registers::Registers = BASE as *const _;

    /// Check if interrupt `i` is enabled on context `c`
    pub fn is_enabled(context: usize, interrupt: u16) -> bool {
        unsafe {
            (*Self::REGS).enable[context].enables[(interrupt / 32) as usize].read()
                & 1 << (interrupt % 32)
                != 0
        }
    }

    /// Enable interrupt `i` for context `c`
    pub fn enable(context: usize, interrupt: u16) {
        unsafe {
            (*Self::REGS).enable[context].enables[(interrupt / 32) as usize]
                .modify(|v| v | 1 << (interrupt % 32))
        }
    }

    /// Disable interrupt `i` for context `c`
    pub fn disable(context: usize, interrupt: u16) {
        unsafe {
            (*Self::REGS).enable[context].enables[(interrupt / 32) as usize]
                .modify(|v| v & !(1 << (interrupt % 32)))
        }
    }

    /// Enable/Disable interrupt `i` for context `c`
    pub fn toggle(context: usize, interrupt: u16) {
        unsafe {
            (*Self::REGS).enable[context].enables[(interrupt / 32) as usize]
                .modify(|v| v ^ 1 << (interrupt % 32))
        }
    }

    /// Get a 32bit register in enable of context `c`
    pub fn get_enable(context: usize, index: usize) -> u32 {
        unsafe { (*Self::REGS).enable[context].enables[index].read() }
    }

    /// Set a 32bit register in enable of context `c`
    pub fn set_enable(context: usize, index: usize, value: u32) {
        unsafe { (*Self::REGS).enable[context].enables[index].modify(|_| value) }
    }

    /// Clear a 32bit register in enable of context `C`
    pub fn clear_enable(context: usize, index: usize) {
        unsafe { (*Self::REGS).enable[context].enables[index].modify(|_| 0) }
    }

    /// Get interrupt `i` priority
    pub fn get_priority(interrupt: u16) -> Priority<BIT> {
        unsafe { (*Self::REGS).priority[interrupt as usize].read().into() }
    }

    /// Set `i` priority
    pub fn set_priority(interrupt: u16, priority: Priority<BIT>) {
        unsafe { (*Self::REGS).priority[interrupt as usize].write(priority.into()) }
    }

    /// Get threshold for context `c`
    pub fn get_threshold(context: usize) -> Priority<BIT> {
        unsafe { (*Self::REGS).context[context].threshold.read().into() }
    }

    /// Set threshold for context `c`
    pub fn set_threshold(context: usize, threshold: Priority<BIT>) {
        unsafe {
            (*Self::REGS).context[context]
                .threshold
                .write(threshold.into())
        }
    }

    /// Check if interrupt `i` is pending
    pub fn is_pending(interrupt: u16) -> bool {
        unsafe {
            (*Self::REGS).pending[(interrupt / 32) as usize].read() & 1 << (interrupt % 32) != 0
        }
    }

    /// Get context address
    pub fn context_address(context: usize) -> usize {
        unsafe { &(*Self::REGS).context[context] as *const _ as usize }
    }

    /// Claim interrupt
    pub fn claim(context: usize) -> Option<u16> {
        let irq = unsafe { (*Self::REGS).context[context].claim_complete.read() };
        match irq {
            0 => None,
            _ => Some(irq as u16),
        }
    }

    /// Complete interrupt
    pub fn complete(context: usize, interrupt: u16) {
        unsafe {
            (*Self::REGS).context[context]
                .claim_complete
                .write(interrupt as u32)
        }
    }

    /// Read context reserved
    ///
    /// `context.reserved[address]`, which is after claim/complete
    pub fn get_context_reserved(context: usize, address: usize) -> u32 {
        unsafe { (*Self::REGS).context[context].reserved[address].read() }
    }

    /// Write context reserved
    ///
    /// `context.reserved[address]`, which is after claim/complete
    pub fn set_context_reserved(context: usize, address: usize, value: u32) {
        unsafe { (*Self::REGS).context[context].reserved[address].write(value) }
    }
}
