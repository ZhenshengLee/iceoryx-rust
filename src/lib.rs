#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

// pub const DDS_DOMAIN_DEFAULT                : u32 = 0xffffffff as u32;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
