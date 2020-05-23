# dnsmasq-plus

Official website of `dnsmasq`: http://www.thekelleys.org.uk/dnsmasq/doc.html

This project fork `dnsmasq` version `v2.80` and improve with some special functionalities.

Using `rust` ffi.

## Requirement

- gcc

- rust

  https://www.rust-lang.org/tools/install

## Install

```bash
make
```

The bin is `src/dnsmasq`.

## Plus

- [re-address](#re-address)
- [re-server](#re-server)

### re-address

Now you can use `re-address` to match a regexp of domain, just as:

```ini
re-address=/^double-click\.net$/127.0.0.1
re-address=/^.*$/127.0.0.2
```

**Notice that `address` or `server` is always privileged than `re-address` or `re-server` when matching a query domain, and above `re-address` or `re-server` is privileged than below.**

### re-server

Regexp version of `server`.

```ini
re-server=/^double-click\.net$/8.8.8.8
```

**Notice that `address` or `server` is always privileged than `re-address` or `re-server` when matching a query domain, and above `re-address` or `re-server` is privileged than below.**

## License

Extends [GPL v2](COPYING) or [GPL v3](COPYING-v3).
