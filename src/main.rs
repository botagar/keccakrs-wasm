extern crate keccakrs;
use keccakrs::*;

use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

fn main() {}

#[no_mangle]
pub fn keccak256(c_input: *mut c_char) -> *mut c_char {
    let mut keccak = keccakrs::new_keccak1600_256();
    let input = &mut cpointer_to_string(c_input);

    keccak.injest(input);
    let mut raw_hash = keccak.hash();
    let hash = vec_to_hex_string(raw_hash);

    CString::new(hash).unwrap().into_raw()
}

fn cpointer_to_string(i: *mut c_char) -> String {
  unsafe {
      CStr::from_ptr(i).to_string_lossy().into_owned()
  }
}

fn vec_to_hex_string(bytes: Vec<u8>) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02x}", b))
                               .collect();
  strs.join("")
}