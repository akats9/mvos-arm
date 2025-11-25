pub mod c {
    unsafe extern "C" {
        pub(crate) unsafe fn c_init_xhci() -> i32;
    }
}