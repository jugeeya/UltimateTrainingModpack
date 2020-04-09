extern "C" {
    fn skyline_tcp_send_raw(bytes: *const u8, usize: u64);
}

pub fn log(message: &str) {
    unsafe {
        skyline_tcp_send_raw(message.as_bytes().as_ptr(), message.as_bytes().len() as _);
    }
}

/// Prints to the standard output, with a newline.
#[macro_export] macro_rules! println {
    () => {
        $crate::log();
    };
    ($($arg:tt)*) => {
        {
            use $crate::alloc::format;
            $crate::logging::log(&format!(
                $($arg)*
            ));
        }
    };
}
