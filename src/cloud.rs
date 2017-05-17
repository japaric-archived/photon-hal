use core::cell::Cell;
use core::{mem, ptr};

use photon_core::Cloud;
use static_ref::Ref;

use {String, ll};

/// Registers a new cloud function with the given `name`
///
/// **NOTE** Only up to 15 cloud functions can be registered
pub fn function(
    name: &str,
    f: extern "C" fn(&String, Cloud) -> i32,
) -> Result<(), ()> {
    // the max length of the variable name is 12 characters
    if name.len() > 12 {
        return Err(());
    }

    // null terminated string
    let mut buffer = [0; 13];
    buffer[..name.len()].copy_from_slice(name.as_bytes());

    if unsafe {
        ll::spark_function(buffer.as_ptr(), mem::transmute(f), ptr::null_mut())
    }
    {
        Ok(())
    } else {
        Err(())
    }
}

/// Registers a new cloud variable with the given `name`
///
/// **NOTE** Only up to 20 cloud variables can be registered
pub fn variable<V>(name: &str, variable: V) -> Result<(), ()>
where
    V: IntRef,
{
    // the max length of the variable name is 12 characters
    if name.len() > 12 {
        return Err(());
    }

    // null terminated string
    let mut buffer = [0; 13];
    buffer[..name.len()].copy_from_slice(name.as_bytes());

    if unsafe {
        ll::spark_variable(
            buffer.as_ptr(),
            variable.as_ptr() as *const _,
            ll::Spark_Data_TypeDef::CLOUD_VAR_INT,
            ptr::null_mut(),
        )
    }
    {
        Ok(())
    } else {
        Err(())
    }
}

/// Implementation detail. Do not implement this trait.
pub unsafe trait IntRef {
    fn as_ptr(self) -> *const i32;
}

unsafe impl<'a> IntRef for Ref<'a, i32> {
    fn as_ptr(self) -> *const i32 {
        &*self
    }
}

unsafe impl<'a> IntRef for Ref<'a, Cell<i32>> {
    fn as_ptr(self) -> *const i32 {
        (*self).as_ptr()
    }
}
