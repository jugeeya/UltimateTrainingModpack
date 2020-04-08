use crate::alloc::string::String;
use crate::alloc::vec::Vec;

pub struct HookInfo {
    pub name: Option<String>,
    pub offset: Option<u64>,
    pub symbol: Option<String>,
    pub inline: bool
}

pub struct Hook {
    pub ptr: *const (),
    pub info: &'static HookInfo
}

impl Hook {
    pub fn install(&self) {
        todo!()
    }
}

pub struct Hooks(pub Vec<Hook>);

impl Hooks {
    pub fn install_hooks(&self) {
        for hook in &self.0 {
            hook.install();
        }
    }
}

#[macro_export] macro_rules! new_hook {
    ($path:path, $info:path) => {
        $crate::hooks::Hook {
            ptr: $path as *const (),
            info: &$info
        }
    };
}
