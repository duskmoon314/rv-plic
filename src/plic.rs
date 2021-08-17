#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x1000 - Interrupt Priorities"]
    pub priority: [crate::Reg<priority::PRIORITY_SPEC>; 1024],
    #[doc = "0x1000..0x1080 - Interrupt Pending"]
    pub pending: [crate::Reg<pending::PENDING_SPEC>; 32],
    _reserved2: [u8; 0x0f80],
    #[doc = "0x2000..0x1f2000 - Interrupt Enable on context `c`"]
    pub enable_c: [ENABLE_C; 15872],
    #[cfg(not(feature = "ctrl_1ffffc"))]
    _reserved3: [u8; 0xe000],
    #[cfg(feature = "ctrl_1ffffc")]
    _reserved3: [u8; 0xdffc],
    #[cfg(feature = "ctrl_1ffffc")]
    #[doc = "0x1ffffc - PLIC Controler"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x200000 - Threshold and Claim/Complete of context"]
    pub context: crate::ArrayProxy<CONTEXT, 15872, 0x1000>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct ENABLE_C {
    #[doc = "0x00..0x80 - Interrupt Enable bits of source `s`"]
    pub enable_s: [crate::Reg<self::enable_c::enable_s::ENABLE_S_SPEC>; 32],
}
#[doc = r"Register block"]
#[doc = "Interrupt Enable on context `c`"]
pub mod enable_c;
#[doc = r"Register block"]
#[repr(C)]
pub struct CONTEXT {
    #[doc = "0x00 - Priority Thresholds on context `c`"]
    pub threshold: crate::Reg<self::context::threshold::THRESHOLD_SPEC>,
    #[doc = "0x04 - Claim/Complete on context `c`"]
    pub claim_complete: crate::Reg<self::context::claim_complete::CLAIM_COMPLETE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Threshold and Claim/Complete of context"]
pub mod context;
#[doc = "priority register accessor: an alias for `Reg<PRIORITY_SPEC>`"]
pub type PRIORITY = crate::Reg<priority::PRIORITY_SPEC>;
#[doc = "Interrupt Priorities"]
pub mod priority;
#[doc = "pending register accessor: an alias for `Reg<PENDING_SPEC>`"]
pub type PENDING = crate::Reg<pending::PENDING_SPEC>;
#[doc = "Interrupt Pending"]
pub mod pending;
#[cfg(feature = "ctrl_1ffffc")]
#[doc = "control register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[cfg(feature = "ctrl_1ffffc")]
#[doc = "PLIC Controler"]
pub mod control;
