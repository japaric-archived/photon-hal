//! Low level bindings to the Particle HAL

#![deny(warnings)]
#![no_std]

/// C types
pub mod ctypes;

/// Delay
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod delay;

/// General purpose input/output
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod gpio;
