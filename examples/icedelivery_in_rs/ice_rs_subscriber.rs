// SPDX-License-Identifier: Apache-2.0

use iceoryx_rs_c::*;
use std::ffi::{CStr, CString};

use std::os::raw::{c_double, c_uint, c_void};

use std::error::Error;
use std::thread;
use std::time::Duration;

#[repr(C)]
pub struct RadarObject {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

pub const APP_NAME: &str = "iox-rs-subscriber";

fn main() {
    unsafe {
        let appname_cstr = CString::new(APP_NAME).unwrap();
        iox_runtime_init(appname_cstr.as_ptr());
    }
}
