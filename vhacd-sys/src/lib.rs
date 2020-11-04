#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Drop for crate::IVHACD_Parameters {
    fn drop(&mut self) {
        unsafe {
            crate::IVHACD_FreeUserCallback(self.m_callback);
            crate::IVHACD_FreeUserLogger(self.m_logger);
        }
    }
}
