use regex::Regex;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_void},
    panic::catch_unwind,
    ptr::null,
};

/// Parse and add regexp to global.
#[no_mangle]
pub extern "C" fn dnsmasq_plus_parse_regex(regexp: *const c_char) -> *const c_void {
    let result = catch_unwind(|| parse_regex(regexp));
    match result {
        Ok(regex) => Box::into_raw(Box::new(regex)) as *const c_void,
        Err(e) => {
            let s = e
                .downcast::<String>()
                .unwrap_or_else(|_| Box::new("<unknown>".to_string()));
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
pub extern "C" fn dnsmasq_plus_hostname_is_match(
    regex: *const c_void,
    query_domain: *const c_char,
) -> i32 {
    let result = catch_unwind(|| hostname_is_match(regex, query_domain));
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

fn hostname_is_match(regex: *const c_void, query_domain: *const c_char) -> bool {
    let regex: Box<Regex> = unsafe { Box::from_raw(regex as *mut Regex) };
    let query_domain = unsafe { CStr::from_ptr(query_domain) };
    let query_domain = query_domain.to_str().expect("convert to utf8 str");
    let b = regex.is_match(query_domain);
    let _ = Box::into_raw(regex);
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_lib() {
        let prt = dnsmasq_plus_parse_regex(CString::new(r"^double-click\.net$").unwrap().as_ptr());
        assert_eq!(
            dnsmasq_plus_hostname_is_match(
                prt,
                CString::new(r"double-click.net").unwrap().as_ptr()
            ),
            1
        );
        assert_eq!(
            dnsmasq_plus_hostname_is_match(prt, CString::new(r"unknow.net").unwrap().as_ptr()),
            0
        );
    }
}
