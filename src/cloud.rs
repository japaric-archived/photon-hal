use core::ptr;

use ll;

pub fn variable(name: &str, variable: &'static i32) -> Result<(), ()> {
    // null terminated string
    // the max length of the variable name is 12 characters
    let mut buffer = [0; 13];
    buffer[..12][..name.len()].copy_from_slice(name.as_bytes());

    if unsafe {
        ll::spark_variable(
            buffer.as_ptr() as *const _ as *const _,
            variable as *const _ as *const _,
            ll::Spark_Data_TypeDef::CLOUD_VAR_INT,
            ptr::null_mut()
        )
    }
    {
        Ok(())
    } else {
        Err(())
    }
}
