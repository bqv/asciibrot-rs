
extern crate libc;

use self::libc::{c_ulong, c_ushort, O_RDWR, O_NOCTTY, ioctl, open, close};

use std::io::{Result, Error, ErrorKind};
use std::ffi::CString;

#[repr(C)]
struct winsize {
    ws_row: c_ushort,     /* rows, in characters */
    ws_col: c_ushort,     /* columns, in characters */
    ws_xpixel: c_ushort,  /* horizontal size, pixels */
    ws_ypixel: c_ushort   /* vertical size, pixels */
}

const TIOCGWINSZ: c_ulong = 0x005413;

pub fn get_winsize() -> Result<(isize, isize)> {
    let w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    let f = unsafe { open(CString::new("/dev/tty").unwrap().into_raw(), O_RDWR|O_NOCTTY) };
    let r = unsafe { ioctl(f, TIOCGWINSZ, &w) };
    unsafe { close(f) };

    match r {
        0 => Ok((w.ws_col as isize, w.ws_row as isize)),
        _ => {
            return Err(Error::new(ErrorKind::NotFound, "Failed to determine window size"))
        }
    }
}

#[test]
fn winsize_has_valid_width_and_height() {
    let (width, height) = get_winsize().unwrap();
    assert!(width > 0);
    assert!(height > 0);
}

