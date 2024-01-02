use std::mem::size_of;

use winapi::shared::basetsd::DWORD32;
use winapi::um::winbase::{NOPARITY, ONESTOPBIT};

use super::common::*;
use super::port::Port;

pub struct PortBuilder<'a> {
  filename: &'a str,
  handle: HANDLE,
  baud: u32,
  timeout_ms: u32,
}

impl<'a> PortBuilder<'a> {
   pub fn new(filename: &'a str) -> std::io::Result<PortBuilder<'a>> {
    let cstr_filename = CString::new(filename)?;

    let handle = unsafe {
      CreateFileA(
        cstr_filename.as_ptr(),
        GENERIC_READ | GENERIC_WRITE,
        0,
        null_mut(),
        OPEN_EXISTING,
        FILE_FLAG_OVERLAPPED,
        null_mut()
      )
    };

    if handle == INVALID_HANDLE_VALUE {
      Err(std::io::Error::last_os_error())
    } else {
      Ok(Self{filename, handle, baud: 0u32, timeout_ms: 0u32})
    }
  }
  pub fn baud(self, baud: u32) -> std::io::Result<PortBuilder<'a>> {
    let mut dcb: DCB = unsafe{ std::mem::zeroed() }; /* I don't know if it would cause a disaster */
    dcb.BaudRate = baud;
    dcb.ByteSize = 8;
    dcb.DCBlength = size_of::<DCB>() as DWORD32;
    dcb.Parity = NOPARITY;
    dcb.StopBits = ONESTOPBIT;

    if unsafe { SetCommState(self.handle, &mut dcb) == FALSE } {
      Err(std::io::Error::last_os_error())
    } else {
      Ok(Self{filename: self.filename, handle: self.handle, baud, timeout_ms: self.timeout_ms})
    }
  }
  pub fn timeout(self, timeout_ms: u32) -> std::io::Result<PortBuilder<'a>> {
    let mut timeouts = COMMTIMEOUTS {
      ReadIntervalTimeout : MAXDWORD,
      ReadTotalTimeoutMultiplier : timeout_ms, 
      ReadTotalTimeoutConstant : 0,
      WriteTotalTimeoutMultiplier : timeout_ms,
      WriteTotalTimeoutConstant : 0
    };

    if unsafe { SetCommTimeouts(self.handle, &mut timeouts) != FALSE } {
      Ok(Self{filename: self.filename, handle: self.handle, baud: self.baud, timeout_ms})
    } else {
      Err(std::io::Error::last_os_error())
    }
  }
  #[inline(always)]
  fn flush(&self) -> std::io::Result<()> {
    if unsafe { FlushFileBuffers(self.handle) == FALSE } {
      Err(std::io::Error::last_os_error())
    } else {
      Ok(())
    }
  }
  pub fn build(self) -> std::io::Result<Port> {
    self.flush()?;
    Ok(
      Port::new( 
        self.handle, 
        self.baud, 
        self.timeout_ms
      )
    )
  }
}