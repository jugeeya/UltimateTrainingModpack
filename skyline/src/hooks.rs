use crate::alloc::string::String;

pub struct HookInfo {
    pub fn_name: &'static str,
    pub name: Option<String>,
    pub offset: Option<u64>,
    pub symbol: Option<String>,
    pub inline: bool
}

pub struct Hook {
    pub ptr: *const (),
    pub info: &'static HookInfo,
}

unsafe impl Sync for Hook {

}

impl Hook {
    pub fn install(&self) {
        todo!()
    }
}

#[allow(improper_ctypes)]
extern "C" {
    static __hook_array_start: Hook;
    static __hook_array_end: Hook;
}

pub fn iter_hooks() -> impl Iterator<Item = &'static Hook> {
    let hook_start = unsafe {&__hook_array_start as *const Hook};
    let hook_end = unsafe {&__hook_array_end as *const Hook};

    let hook_count = ((hook_start as usize) - (hook_end as usize)) / core::mem::size_of::<Hook>();

    crate::println!("hook_count: {}", hook_count);
    crate::println!("hook_start: {:?}", hook_start);
    crate::println!("hook_end: {:?}", hook_start);

    unsafe {
        core::slice::from_raw_parts(
            hook_start,
            hook_count
        )
    }.iter()
}

