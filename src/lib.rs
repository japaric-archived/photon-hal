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

/// Serial communication (not via USB)
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod usart;

/// USB Virtual COM Port and HID device
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod usb;

/// Device and platform identifiers
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod deviceid;
