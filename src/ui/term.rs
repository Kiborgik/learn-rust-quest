//! Platform calls: enable colors on Windows, read the terminal width.
//! Each OS gets a small FFI block; others fall back to "unknown".

pub use imp::{enable_ansi, width};

#[cfg(windows)]
mod imp {
    use core::ffi::c_void;

    const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    #[repr(C)]
    #[derive(Default)]
    struct Coord {
        x: i16,
        y: i16,
    }

    #[repr(C)]
    #[derive(Default)]
    struct SmallRect {
        left: i16,
        top: i16,
        right: i16,
        bottom: i16,
    }

    /// CONSOLE_SCREEN_BUFFER_INFO
    #[repr(C)]
    #[derive(Default)]
    struct Info {
        size: Coord,
        cursor: Coord,
        attributes: u16,
        window: SmallRect,
        max_size: Coord,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetStdHandle(n: u32) -> *mut c_void;
        fn GetConsoleMode(h: *mut c_void, mode: *mut u32) -> i32;
        fn SetConsoleMode(h: *mut c_void, mode: u32) -> i32;
        fn GetConsoleScreenBufferInfo(h: *mut c_void, info: *mut Info) -> i32;
    }

    /// Classic Windows consoles need VT processing switched on before ANSI
    /// escape codes render as colors.
    pub fn enable_ansi() {
        // SAFETY: plain Win32 calls with a valid out-pointer; failures
        // return 0 and leave the console as it was.
        unsafe {
            let h = GetStdHandle(STD_OUTPUT_HANDLE);
            let mut mode = 0;
            if GetConsoleMode(h, &mut mode) != 0 {
                SetConsoleMode(h, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }

    /// The console window's width in columns.
    pub fn width() -> Option<usize> {
        let mut info = Info::default();
        // SAFETY: `info` is a valid, writable struct with the
        // CONSOLE_SCREEN_BUFFER_INFO layout. Failure returns 0.
        let ok = unsafe { GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut info) };
        let width = info.window.right - info.window.left + 1;
        (ok != 0 && width > 0).then_some(width as usize)
    }
}

#[cfg(all(
    unix,
    any(
        all(
            target_os = "linux",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "aarch64",
                target_arch = "arm",
                target_arch = "riscv64"
            )
        ),
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
    )
))]
mod imp {
    use std::ffi::{c_int, c_ulong};

    #[cfg(target_os = "linux")]
    const TIOCGWINSZ: c_ulong = 0x5413;
    #[cfg(not(target_os = "linux"))]
    const TIOCGWINSZ: c_ulong = 0x4008_7468;

    /// struct winsize
    #[repr(C)]
    #[derive(Default)]
    struct WinSize {
        rows: u16,
        cols: u16,
        x_pixels: u16,
        y_pixels: u16,
    }

    unsafe extern "C" {
        fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    }

    /// Unix terminals understand ANSI codes already.
    pub fn enable_ansi() {}

    /// The terminal's width in columns, if stdout is a terminal.
    pub fn width() -> Option<usize> {
        let mut ws = WinSize::default();
        // SAFETY: TIOCGWINSZ writes one `struct winsize` (four u16s) to the
        // pointer, which points at a valid WinSize. Failure returns -1.
        let ok = unsafe { ioctl(1, TIOCGWINSZ, &mut ws as *mut WinSize) };
        (ok == 0 && ws.cols > 0).then_some(ws.cols as usize)
    }
}

#[cfg(not(any(
    windows,
    all(
        unix,
        any(
            all(
                target_os = "linux",
                any(
                    target_arch = "x86_64",
                    target_arch = "x86",
                    target_arch = "aarch64",
                    target_arch = "arm",
                    target_arch = "riscv64"
                )
            ),
            target_os = "macos",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
        )
    )
)))]
mod imp {
    pub fn enable_ansi() {}

    pub fn width() -> Option<usize> {
        None
    }
}
