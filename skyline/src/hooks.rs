use crate::alloc::string::String;

pub struct HookInfo {
    /// Name of the function being used as the override
    pub fn_name: &'static str,

    /// User-given name of what the hook represents
    pub name: Option<String>,

    /// Offset of where to install the hook
    pub offset: Option<u64>,

    /// Symbol of where to install the hook
    pub symbol: Option<String>,

    /// Whether or not this is an inline hook
    pub inline: bool
}

/// Type for representing a hook for this plugin
pub struct Hook {
    /// Pointer to the overloading function
    pub ptr: *const (),

    /// Info needed to identify and install this hook
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

/// Iterate over the loaded hooks for this plugin
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

