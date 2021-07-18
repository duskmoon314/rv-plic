/*!
Platform Level Interrupt Controller Registers

REF: [RISC-V Platform-Level Interrupt Controller Specification](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc)
*/

use volatile_register::RW;

/// Register Block
#[repr(C)]
pub struct Registers {
    /// - base + 0x000000: Reserved (interrupt source 0 does not exist)
    /// - base + 0x000004: Interrupt source 1 priority
    /// - base + 0x000008: Interrupt source 2 priority
    /// - ...
    /// - base + 0x000FFC: Interrupt source 1023 priority
    pub priority: [RW<u32>; 1024],

    /// - base + 0x001000: Interrupt Pending bit 0-31
    /// - base + 0x00107C: Interrupt Pending bit 992-1023
    pub pending: [RW<u32>; 32],

    _padding_1: [u32; 992],

    /// - base + 0x002000: Enable bits for sources 0-31 on context 0
    /// - base + 0x002004: Enable bits for sources 32-63 on context 0
    /// - ...
    /// - base + 0x00207F: Enable bits for sources 992-1023 on context 0
    /// - base + 0x002080: Enable bits for sources 0-31 on context 1
    /// - base + 0x002084: Enable bits for sources 32-63 on context 1
    /// - ...
    /// - base + 0x0020FF: Enable bits for sources 992-1023 on context 1
    /// - base + 0x002100: Enable bits for sources 0-31 on context 2
    /// - base + 0x002104: Enable bits for sources 32-63 on context 2
    /// - ...
    /// - base + 0x00217F: Enable bits for sources 992-1023 on context 2
    /// - ...
    /// - base + 0x1F1F80: Enable bits for sources 0-31 on context 15871
    /// - base + 0x1F1F84: Enable bits for sources 32-63 on context 15871
    /// - base + 0x1F1FFF: Enable bits for sources 992-1023 on context 15871
    pub enable: [Enables; 15872],

    _padding_2: [u32; 14336],

    /// - base + 0x200000: Priority threshold for context 0
    /// - base + 0x200004: Claim/complete for context 0
    /// - base + 0x200008: Reserved
    /// - ...
    /// - base + 0x200FFC: Reserved
    /// - base + 0x201000: Priority threshold for context 1
    /// - base + 0x201004: Claim/complete for context 1
    /// - ...
    /// - base + 0x3FFF000: Priority threshold for context 15871
    /// - base + 0x3FFF004: Claim/complete for context 15871
    /// - base + 0x3FFF008: Reserved
    pub context: [Context; 15872],
}

/// Enable Bitmap
#[repr(C)]
pub struct Enables {
    /// 0x00 Interrupt Source #0 to #31 Enable Bits for context
    /// ...
    /// 0x7F Interrupt Source #992 to #1023 Enable Bits for context
    pub enables: [RW<u32>; 32],
}

/// Context
#[repr(C)]
pub struct Context {
    /// 0x0 prority threshold for context
    pub threshold: RW<u32>,
    /// 0x4 claim/complete for context
    pub claim_complete: RW<u32>,
    pub reserved: [RW<u32>; 1022],
}
