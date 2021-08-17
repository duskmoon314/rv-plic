/*!
Platform Level Interrupt Controller Registers

REF: [RISC-V Platform-Level Interrupt Controller Specification](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc)
*/

// macro_rules! cast {
//     ($expr:expr) => {
//         unsafe { &mut *(($expr) as *mut crate::registers::Registers) };
//     };
// }

use tock_registers::registers::ReadWrite;

#[cfg(feature = "ctrl_1ffffc")]
register_bitfields! [
    u32,
    Control [
        S_PER 0,
    ],
];

register_structs! {
    pub(crate) Enable {
        (0x0000 => pub enable: [ReadWrite<u32>; 32]),
        (0x0080 => @END),
    },

    pub(crate) Context {
        (0x0000 => pub threshold: ReadWrite<u32>),
        (0x0004 => pub claim_complete: ReadWrite<u32>),
        (0x0008 => _reserved),
        (0x1000 => @END),
    },

    pub(crate) Registers {
        /// - base + 0x000000: Reserved (interrupt source 0 does not exist)
        /// - base + 0x000004: Interrupt source 1 priority
        /// - base + 0x000008: Interrupt source 2 priority
        /// - ...
        /// - base + 0x000FFC: Interrupt source 1023 priority
        (0x0000000 => pub priority: [ReadWrite<u32>; 1024]),

        /// - base + 0x001000: Interrupt Pending bit 0-31
        /// - base + 0x00107C: Interrupt Pending bit 992-1023
        (0x001000 => pub pending: [ReadWrite<u32>; 32]),

        (0x001080 => _reserved1),

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
        (0x002000 => pub enable: [Enable; 15872]),

        (0x1f2000 => _reserved2),

        /// Control Register at base + 0x1FFFFC
        ///
        /// - Xuantie C906
        #[cfg(feature = "ctrl_1ffffc")]
        (0x1ffffc => pub control: ReadWrite<u32, Control::Register>),
        #[cfg(not(feature = "ctrl_1ffffc"))]
        (0x1ffffc => _reserved3),

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
        (0x200000 => pub context: [Context; 15872]),

        (0x4000000 => @END),
    }
}
