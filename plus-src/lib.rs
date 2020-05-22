use regex::Regex;
use std::{
    ffi::CStr,
    mem::forget,
    os::raw::{c_char, c_void},
    ptr::null,
};

/// Parse and add regexp to global.
#[no_mangle]
pub extern "C" fn dnsmasq_plus_parse_regex(regexp: *const c_char) -> *const c_void {
    match parse_regex(regexp) {
        Ok(regex) => Box::into_raw(Box::new(regex)) as *const c_void,
        Err(e) => {
            eprintln!("dnsmasq: there is an error when calling the function `dnsmasq_plus_parse_regex`: {:?}", e);
            null()
        }
    }
}

fn parse_regex(regexp: *const c_char) -> anyhow::Result<Regex> {
    let regex = unsafe { CStr::from_ptr(regexp) };
    let regex = regex.to_str()?;
    Ok(Regex::new(regex)?)
}

/// Use regexp to match a domain.
#[no_mangle]
pub extern "C" fn dnsmasq_plus_hostname_is_match(
    regex: *const c_void,
    query_domain: *const c_char,
) -> i32 {
    match hostname_is_match(regex, query_domain) {
        Ok(b) => b as i32,
        Err(e) => {
            eprintln!("dnsmasq: there is an error when calling the function `dnsmasq_plus_hostname_is_match`: {:?}", e);
            0
        }
    }
}

fn hostname_is_match(regex: *const c_void, query_domain: *const c_char) -> anyhow::Result<bool> {
    let regex: Box<Regex> = unsafe { Box::from_raw(regex as *mut Regex) };
    let query_domain = unsafe { CStr::from_ptr(query_domain) };
    let query_domain = query_domain.to_str()?;
    let b = regex.is_match(query_domain);
    forget(regex);
    Ok(b)
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
