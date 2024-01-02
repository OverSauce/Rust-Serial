use winapi::{um::fileapi::{WriteFile, ReadFile}, shared::minwindef::{LPCVOID, DWORD, LPVOID}};
use super::common::*;

pub enum BytesSent {
  AllBytes(u32),
  NotAllBytes(u32),
  NoBytes,
}

#[allow(unused)]
pub struct Port {
  handle: HANDLE,
  baud: u32,
  timeout_ms: u32,
}

impl Port {
  #[inline(always)]
  pub fn new(handle: HANDLE, baud: u32, timeout_ms: u32) -> Port {
    Self {handle, baud, timeout_ms}
  }
  #[allow(dead_code)]
  pub fn write(&self, buffer: *const u8, buffer_size: u32) -> BytesSent {
    let mut written: DWORD = 0;
    
    if unsafe { 
      WriteFile(self.handle,
         buffer as LPCVOID,
          buffer_size, 
          &mut written, 
          null_mut()
        ) == FALSE 
      } {
      BytesSent::NoBytes
    } else {
      if written != buffer_size as DWORD {
        BytesSent::NotAllBytes(written)
      } else {
        BytesSent::AllBytes(written)
      }
    }
  }
  #[allow(dead_code)]
  pub fn read(&self, buffer: *mut u8, buffer_size: u32) -> u32 {
    let mut received: DWORD = 0;

    if unsafe { 
      ReadFile(
        self.handle,
        buffer as LPVOID,
        buffer_size, 
        &mut received, 
        null_mut()) == FALSE 
      } {
        0
    } else {
        received
    }
  }
}