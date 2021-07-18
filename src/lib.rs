//! RISC-V Platform Level Interrupt Controller

#![no_std]

extern crate volatile_register;

pub mod plic;
pub mod registers;

pub use plic::{Priority, PLIC};
