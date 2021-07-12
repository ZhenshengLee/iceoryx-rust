// SPDX-License-Identifier: Apache-2.0

use iceoryx_rs_c::*;
use libc::c_char;
use std::ffi::{CStr, CString};

use std::os::raw::{c_double, c_uint, c_void};

use std::{mem, ptr, thread};

use std::error::Error;
use std::time::Duration;

#[repr(C)]
pub struct RadarObject {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

fn main() {
    unsafe {
        iox_runtime_init(CString::new("iox-rs-publisher").unwrap().into_raw());

        let mut options = mem::MaybeUninit::<iox_pub_options_t>::uninit();
        iox_pub_options_init(options.as_mut_ptr());
        (*(options.as_mut_ptr())).historyCapacity = 10;
        (*(options.as_mut_ptr())).nodeName =
            CString::new("iox-rs-publisher-node").unwrap().into_raw();

        let mut publisher_storage = mem::MaybeUninit::<iox_pub_storage_t>::zeroed();

        let publisher = iox_pub_init(
            publisher_storage.as_mut_ptr(),
            CString::new("Radar").unwrap().into_raw(),
            CString::new("FrontLeft").unwrap().into_raw(),
            CString::new("Object").unwrap().into_raw(),
            options.as_mut_ptr(),
        );

        let mut ct = 0.0;
        loop {
            let mut user_payload = ptr::null_mut();
            iox_pub_loan_chunk(
                publisher,
                &mut user_payload,
                mem::size_of::<RadarObject>() as u32,
            );

            let mut sample = ptr::null_mut();
            sample = user_payload as *mut RadarObject;

            (*sample).x = ct;
            (*sample).y = ct;
            (*sample).z = ct;

            ct = ct + 1.0;

            iox_pub_publish_chunk(publisher, user_payload);

            println!("send a copy...");
            thread::sleep(Duration::from_millis(400));
        }
    }
}
