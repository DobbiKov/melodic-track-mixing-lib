#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    // all the items produced by bindgen go **inside** this module
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod audio_analyze;
pub mod key_detect;
pub mod types;

#[cfg(test)]
pub mod tests;
