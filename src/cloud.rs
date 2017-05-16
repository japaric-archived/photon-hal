use core::ptr;

use {String, ll};

pub fn function(
    name: &str,
    f: extern "C" fn(&String) -> i32,
) -> Result<(), ()> {
    // the max length of the variable name is 12 characters
    if name.len() > 12 {
        return Err(());
    }

    // null terminated string
    let mut buffer = [0; 13];
    buffer[..name.len()].copy_from_slice(name.as_bytes());

    if unsafe { ll::spark_function(buffer.as_ptr(), f, ptr::null_mut()) } {
        Ok(())
    } else {
        Err(())
    }
}

pub fn variable(name: &str, variable: &'static i32) -> Result<(), ()> {
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
            variable as *const _ as *const _,
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
