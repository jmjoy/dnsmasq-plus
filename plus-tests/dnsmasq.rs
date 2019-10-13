use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use std::panic::{catch_unwind, resume_unwind};

const PORT: &'static str = "5354";

#[test]
fn test_dnsmasq() {
    Command::new("make").spawn().unwrap().wait().unwrap();

    let child = start_dnsmasq("plus-tests/0.config");
    let result = catch_unwind(|| {
        assert!(test_dig_result("localhost.local", "127.0.0.1"));
        assert!(test_dig_result("me.localhost.local", "127.0.0.1"));
        assert!(test_dig_result("test.local", "127.0.0.2"));
        assert!(test_dig_result("test.test.local", "127.0.0.2"));
        assert!(test_dig_result("test.net", "127.0.0.3"));
        assert!(test_dig_result("test.test.net", "127.0.0.4"));
    });
    kill_dnsmasq(child);
    match result {
        Ok(_) => {},
        Err(e) => resume_unwind(e),
    }
}

fn start_dnsmasq(config_file: &str) -> Child {
    let mut dnsmasq = Command::new("./src/dnsmasq");
    dnsmasq.arg("-d")
        .arg("-C")
        .arg(config_file)
        .arg("-p")
        .arg(PORT);
    let child = dnsmasq.spawn().unwrap();
    thread::sleep(Duration::from_secs(2));
    child
}

fn kill_dnsmasq(mut child: Child) {
    child.kill().unwrap();
}

fn test_dig_result(domain: &str, want: &str) -> bool {
    let output = Command::new("dig")
        .arg("@127.0.0.1")
        .arg("-p")
        .arg(PORT)
        .arg("ANY")
        .arg(domain)
        .output()
        .unwrap();
    let content = String::from_utf8(output.stdout).unwrap();
    let mut lines = content.lines();
    let line = loop {
        let line = lines.next().unwrap();
        if line.starts_with(";; ANSWER SECTION:") {
            break lines.next().unwrap();
        }
    };
    eprintln!("{}", line);
    line.ends_with(want)
}
