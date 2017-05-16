//! Low level bindings to the Particle HAL

#![allow(non_camel_case_types)]
#![deny(warnings)]
#![no_std]

use core::{ops, slice};

pub mod cloud;
pub mod ctypes;
pub mod ll;

use ctypes::{c_char, c_uchar, c_uint};

#[repr(C)]
pub struct String {
    /// the actual char array
    buffer: *mut c_char,
    /// the array length minus one (for the '\0')
    capacity: c_uint,
    /// the String length (not counting the '\0')
    len: c_uint,
    /// unused, for future features
    flags: c_uchar,
}

impl ops::Deref for String {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}

pub struct UsbSerial;

impl UsbSerial {
    /// Enables the serial channel with the specified `baud_rate`
    pub fn begin(&self, baud_rate: u32) {
        unsafe { ll::USB_USART_Init(baud_rate) }
    }

    /// Writes binary data to the serial port
    pub fn write(&self, byte: u8) {
        unsafe {
            ll::USB_USART_Send_Data(byte);
        }
    }
}

pub enum PinMode {
    Input,
    InputPulldown,
    InputPullup,
    Output,
}

pub const LED: D7 = D7;

pub struct D0;
pub struct D1;
pub struct D2;
pub struct D3;
pub struct D4;
pub struct D5;
pub struct D6;
pub struct D7;

pub struct A0;
pub struct A1;
pub struct A2;
pub struct A3;
pub struct A4;
pub struct A5;

macro_rules! pin_mode {
    ($pin:ident, $i:expr) => {
        impl $pin {
            pub fn pin_mode(&self, mode: PinMode) {
                match mode {
                    PinMode::Input => unsafe {
                        ll::HAL_Pin_Mode($i, ll::PinMode::INPUT)
                    },
                    PinMode::InputPulldown => unsafe {
                        ll::HAL_Pin_Mode($i, ll::PinMode::INPUT_PULLDOWN)
                    },
                    PinMode::InputPullup => unsafe {
                        ll::HAL_Pin_Mode($i, ll::PinMode::INPUT_PULLUP)
                    },
                    PinMode::Output => unsafe {
                        ll::HAL_Pin_Mode($i, ll::PinMode::OUTPUT)
                    },
                }
            }
        }
    }
}

pin_mode!(D0, 0);
pin_mode!(D1, 1);
pin_mode!(D2, 2);
pin_mode!(D3, 3);
pin_mode!(D4, 4);
pin_mode!(D5, 5);
pin_mode!(D6, 6);
pin_mode!(D7, 7);

macro_rules! digital_write {
    ($pin:ident, $i:expr) => {
        impl $pin {
            pub fn high(&self) {
                unsafe {
                    ll::HAL_GPIO_Write($i, 1)
                }
            }

            pub fn low(&self) {
                unsafe {
                    ll::HAL_GPIO_Write($i, 0)
                }
            }
        }
    }
}

digital_write!(D0, 0);
digital_write!(D1, 1);
digital_write!(D2, 2);
digital_write!(D3, 3);
digital_write!(D4, 4);
digital_write!(D5, 5);
digital_write!(D6, 6);
digital_write!(D7, 7);

/// Returns the ID of the Particle device
pub fn device_id() -> String {
    unsafe { ll::spark_deviceID() }
}

pub fn sleep_ms(us: u32) {
    unsafe { ll::HAL_Delay_Milliseconds(us) }
}

pub fn sleep_us(us: u32) {
    unsafe { ll::HAL_Delay_Microseconds(us) }
}
