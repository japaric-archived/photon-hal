//! Low level bindings to [particle] [HAL][0] (Hardware Abstraction Layer)
//!
//! [particle]: https://www.particle.io/
//! [0]: https://github.com/spark/firmware/tree/develop/hal/inc

#![no_std]

#[allow(non_camel_case_types)]
pub mod delay;
#[allow(non_camel_case_types)]
pub mod gpio;

mod ty;
