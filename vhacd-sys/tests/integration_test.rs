#[test]
fn basic_functionality() {
    unsafe {
        let ptr = vhacd_sys::CreateVHACD();
        assert!(vhacd_sys::IVHACD_IsReady_typed(ptr));
        vhacd_sys::IVHACD_Release(ptr);
    }
}
