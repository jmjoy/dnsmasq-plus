# dnsmasq-plus

Official website of `dnsmasq`: http://www.thekelleys.org.uk/dnsmasq/doc.html

This project fork `dnsmasq` version `v2.80` and improve with some special functionalities.

Use `rust` ffi.

# Requirement

- gcc
- rust

# Install

```bash
make
```

## Plus

- `re-address`

### re-address

Now you can use `re-address` to match a regexp of domain, just as:

```ini
re-address=/^double-click\.net$/127.0.0.1
re-address=/^.*$/127.0.0.2
```

**Notice that `address` or `server` is always privileged than `re-address` when matching a query domain, and above `re-address` is privileged than below.**
