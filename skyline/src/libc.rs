//! Public exports of libc functions
#![allow(non_camel_case_types)]

pub use core::ffi::c_void;

pub type c_char = u8;
pub type time_t = i32;
pub type wchar_t = u16;
pub type c_long = i64;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type in_addr_t = u32;
pub type in_port_t = u16;

pub type mode_t = c_uint;
pub type off_t = i64;
#[repr(C)]
pub struct sem_t { // Unverified
    __size: [c_char; 16],
}
pub type pthread_key_t = c_uint;

#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
}

#[derive(Debug, Clone, Copy)]
pub enum FILE {}

#[derive(Debug, Clone, Copy)]
pub enum DIR {}

extern "C" {
    pub fn malloc(size: size_t) -> *const c_void;
    pub fn free(ptr: *const c_void);
    pub fn calloc(num: size_t, size: size_t) -> *const c_void;
    pub fn realloc(ptr: *const c_void, size: size_t) -> *const c_void;
    // fn aligned_alloc(align: usize, size: usize) -> *const c_void;
}

extern "C" {
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isalpha(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn isblank(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;
    
    pub fn qsort(
        base: *mut c_void,
        num: size_t,
        size: size_t,
        compar: Option<
            unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
        >,
    );
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn fflush(file: *mut FILE) -> c_int;
    pub fn fclose(file: *mut FILE) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn setvbuf(
        stream: *mut FILE,
        buffer: *mut c_char,
        mode: c_int,
        size: size_t,
    ) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn getchar() -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fwrite(
        ptr: *const c_void,
        size: size_t,
        nobj: size_t,
        stream: *mut FILE,
    ) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);

    pub fn perror(s: *const c_char);
    pub fn atoi(s: *const c_char) -> c_int;

    pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    pub fn strtol(
        s: *const c_char,
        endp: *mut *mut c_char,
        base: c_int,
    ) -> c_long;

    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;

    pub fn atexit(cb: extern "C" fn()) -> c_int;

    pub fn getenv(s: *const c_char) -> *mut c_char;

    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(
        dst: *mut c_char,
        src: *const c_char,
        n: size_t,
    ) -> *mut c_char;
    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
    pub fn strncat(
        s: *mut c_char,
        ct: *const c_char,
        n: size_t,
    ) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
    pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strdup(cs: *const c_char) -> *mut c_char;
    pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncasecmp(
        s1: *const c_char,
        s2: *const c_char,
        n: size_t,
    ) -> c_int;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;

    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
    pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;

    pub fn wcslen(buf: *const wchar_t) -> size_t;
    pub fn wcstombs(
        dest: *mut c_char,
        src: *const wchar_t,
        n: size_t,
    ) -> size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(
        dest: *mut c_void,
        src: *const c_void,
        n: size_t,
    ) -> *mut c_void;

    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    pub fn fprintf(
        stream: *mut FILE,
        format: *const c_char,
        ...
    ) -> c_int;
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn snprintf(
        s: *mut c_char,
        n: size_t,
        format: *const c_char,
        ...
    ) -> c_int;
    pub fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int;

    pub fn access(path: *const c_char, amode: c_int) -> c_int;

    pub fn chdir(dir: *const c_char) -> c_int;

    pub fn close(fd: c_int) -> c_int;

    pub fn getpid() -> pid_t;

    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;

    pub fn posix_memalign(
        memptr: *mut *mut c_void,
        align: size_t,
        size: size_t,
    ) -> c_int;

    pub fn rmdir(path: *const c_char) -> c_int;
    pub fn sleep(secs: c_uint) -> c_uint;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t)
        -> ssize_t;

    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;

    pub fn unlink(c: *const c_char) -> c_int;

    pub fn write(
        fd: c_int,
        buf: *const c_void,
        count: size_t,
    ) -> ssize_t;

    pub fn pwrite(
        fd: c_int,
        buf: *const c_void,
        count: size_t,
        offset: off_t,
    ) -> ssize_t;

    pub fn setenv(
        name: *const c_char,
        val: *const c_char,
        overwrite: c_int,
    ) -> c_int;

    pub fn unsetenv(name: *const c_char) -> c_int;

    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;

    pub fn sched_yield() -> c_int;

    pub fn pthread_setspecific(
        key: pthread_key_t,
        value: *const c_void,
    ) -> c_int;

    pub fn mktime(tm: *mut tm) -> time_t;
    pub fn time(time: *mut time_t) -> time_t;
    pub fn gmtime(time_p: *const time_t) -> *mut tm;
    pub fn localtime(time_p: *const time_t) -> *mut tm;
    pub fn difftime(time1: time_t, time0: time_t) -> c_double;

    pub fn putenv(string: *mut c_char) -> c_int;

    pub fn setlocale(
        category: c_int,
        locale: *const c_char,
    ) -> *mut c_char;

    pub fn sem_wait(sem: *mut sem_t) -> c_int;
    pub fn sem_trywait(sem: *mut sem_t) -> c_int;
    pub fn sem_post(sem: *mut sem_t) -> c_int;

    pub fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn getline(
        lineptr: *mut *mut c_char,
        n: *mut size_t,
        stream: *mut FILE,
    ) -> ssize_t;

    pub fn fdopendir(fd: c_int) -> *mut DIR;
}
