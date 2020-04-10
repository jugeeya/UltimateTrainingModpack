#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {

    crate::logging::log("Panic at the Rust lib!\n");

    loop {}
}

global_asm!(include_str!("mod0.s"));

#[no_mangle] pub unsafe extern "C" fn __custom_init() {}
#[no_mangle] pub extern "C" fn __custom_fini() {}

#[macro_export] macro_rules! set_module_name {
    ($lit:literal) => {
        const __SKYLINE_INTERNAL_MODULE_LEN: usize = $lit.len() + 1;
        #[link_section = ".rodata.module_name"]
        pub static __MODULE_NAME: ::skyline::build::ModuleName<__SKYLINE_INTERNAL_MODULE_LEN> =
            ::skyline::build::ModuleName::new(
                ::skyline::skyline_macro::to_null_term_bytes!($lit)
            );
    };
}

#[repr(packed)]
#[allow(unused_variables)]
pub struct ModuleName<const LEN: usize> {
    pub unk: u32,
    pub name_length: u32,
    pub name: [u8; LEN],
}

impl<const LEN: usize> ModuleName<LEN> {
    pub const fn new(bytes: &[u8; LEN]) -> Self {
        Self {
            unk: 0,
            name_length: LEN as u32 - 1,
            name: *bytes,
        }
    }
}

/// one-time setup for skyline
#[doc(hidden)]
#[macro_export] macro_rules! setup {
    () => {
        #[global_allocator]
        pub static ALLOCATOR: $crate::extern_alloc::Allocator = $crate::extern_alloc::Allocator;
    };
}
