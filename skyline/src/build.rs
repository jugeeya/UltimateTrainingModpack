use core::any::Any;

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {

    crate::log("Panic at the Rust lib!\n");

    loop {}
}

global_asm!(include_str!("mod0.s"));

#[no_mangle] pub extern "C" fn __custom_init() {}
#[no_mangle] pub extern "C" fn __custom_fini() {}

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
            name_length: LEN as u32,
            name: *bytes,
        }
    }
}

#[link_section = ".rodata.module_name"]
pub static MODULE_NAME: impl Any = ModuleName::new(b"no_std_test\0");


