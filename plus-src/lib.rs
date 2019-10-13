#![crate_type = "staticlib"]

use regex::Regex;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::panic::catch_unwind;
use std::ptr::null;

/// Parse and add regexp to global.
#[no_mangle]
pub extern "C" fn dnsmasq_plus_parse_regex(regexp: *const c_char) -> *const c_void {
    let result = catch_unwind(|| parse_regex(regexp));
    match result {
        Ok(regex) => Box::into_raw(Box::new(regex)) as *const c_void,
        Err(e) => {
            let s = e.downcast::<String>().unwrap_or_else(|_| Box::new("<unknown>".to_string()));
            eprintln!("dnsmasq: there is an error when calling the function `dnsmasq_plus_parse_regex`: {}", s);
            null()
        }
    }
}

fn parse_regex(regexp: *const c_char) -> Regex {
    let regex = unsafe { CStr::from_ptr(regexp) };
    let regex = regex.to_str().expect("convert to utf8 str");
    Regex::new(regex).expect("parse to regex")
}

/// Use regexp to match a domain.
#[no_mangle]
pub extern "C" fn dnsmasq_plus_hostname_is_match(regex: *const c_void, query_domain: *const c_char) -> i32 {
    let result = catch_unwind(|| hostname_is_match(regex, query_domain));
    match result {
        Ok(b) => b as i32,
        Err(e) => {
            let s = e.downcast::<String>().unwrap_or_else(|_| Box::new("<unknown>".to_string()));
            eprintln!("dnsmasq: there is an error when calling the function `dnsmasq_plus_hostname_is_match`: {}", s);
            0
        }
    }
}

fn hostname_is_match(regex: *const c_void, query_domain: *const c_char) -> bool {
    let regex: Box<Regex> = unsafe {
        Box::from_raw(regex as *mut Regex)
    };
    let query_domain = unsafe { CStr::from_ptr(query_domain) };
    let query_domain = query_domain.to_str().expect("convert to utf8 str");
    let b = regex.is_match(query_domain);
    let _ = Box::into_raw(regex);
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dnsmasq_plus_parse_regex_0() {
        let regexp = r"^double-click\.net$";
        let c_regexp = CString::new(regexp).unwrap();
        assert_eq!(dnsmasq_plus_parse_regex(c_regexp.as_ptr()), 1);
        assert_eq!(REGEXP_MAP.len(), 1);
        assert_eq!((*REGEXP_MAP.get(&c_regexp).unwrap()).as_str(), regexp);
    }

    #[test]
    fn test_global_add_regex_0() {
        let regexp = r"??";
        let c_regexp = CString::new(regexp).unwrap();
        assert_eq!(parse_regex(c_regexp.as_ptr()), true);
        assert_eq!(REGEXP_MAP.len(), 1);
        assert_eq!((*REGEXP_MAP.get(&c_regexp).unwrap()).as_str(), regexp);
    }
}


