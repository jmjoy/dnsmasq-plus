#![crate_type = "staticlib"]

use chashmap::CHashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::catch_unwind;

lazy_static! {
    static ref REGEXP_MAP: CHashMap<CString, Regex> = CHashMap::new();
}

/// Parse and add regexp to global.
#[no_mangle]
pub extern "C" fn dnsmasq_plus_global_add_regex(regexp: *const c_char) -> i32 {
    let result = catch_unwind(|| {
        let regex = unsafe { CStr::from_ptr(regexp) };
        let regexp = unsafe { CString::from_raw(regexp as *mut c_char) };
        let regex = regex.to_str().expect("convert to utf8 str");
        let regex = Regex::new(regex).expect("parse to regex");
        REGEXP_MAP.insert(regexp, regex);
        true
    });
    match result {
        Ok(b) => b as i32,
        Err(e) => {
            let s = e
                .downcast::<String>()
                .unwrap_or_else(|_| Box::new("<unknown>".to_string()));
            eprintln!("dnsmasq: there is an error when calling the function `dnsmasq_plus_global_add_regex`: {}", s);
            0
        }
    }
}

/// Use regexp to match a domain.
#[no_mangle]
pub extern "C" fn dnsmasq_plus_hostname_is_match(
    regexp: *const c_char,
    query_domain: *const c_char,
) -> i32 {
    let result = catch_unwind(|| {
        let regexp = unsafe { CStr::from_ptr(regexp) };
        let query_domain = unsafe { CStr::from_ptr(query_domain) };
        let query_domain = query_domain.to_str().expect("convert to utf8 str");
        let regex = REGEXP_MAP.get(regexp).expect("global map get failed");
        regex.is_match(query_domain)
    });
    match result {
        Ok(b) => b as i32,
        Err(e) => {
            let s = e
                .downcast::<String>()
                .unwrap_or_else(|_| Box::new("<unknown>".to_string()));
            eprintln!("dnsmasq: there is an error when calling the function `dnsmasq_plus_hostname_is_match`: {}", s);
            0
        }
    }
}
