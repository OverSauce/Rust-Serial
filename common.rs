#[cfg(windows)] pub extern crate winapi;

pub use std::ffi::CString;
pub use std::ptr::null_mut;
pub use winapi::shared::minwindef::FALSE;
pub use winapi::um::commapi::{SetCommState, SetCommTimeouts};
pub use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING, FlushFileBuffers};
pub use winapi::shared::ntdef::HANDLE;
pub use winapi::um::handleapi::INVALID_HANDLE_VALUE;
pub use winapi::um::winbase::{FILE_FLAG_OVERLAPPED, DCB, COMMTIMEOUTS};
pub use winapi::um::winnt::{GENERIC_READ, GENERIC_WRITE, MAXDWORD};
