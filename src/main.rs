use std::process::Command;
use std::{thread, time};

fn main() {
    let mut cmd = Command::new("nginx");

    if let Ok(mut child) = cmd.args(&["-g", "daemon off;"]).spawn() {
        println!("start nginx");
        let ten_millis = time::Duration::from_secs(86164);
        let now = time::Instant::now();

        thread::sleep(ten_millis);
        assert!(now.elapsed() >= ten_millis);

        child.kill().expect("command wasn't running");
        println!("stop nginx");
    } else {
        println!("nginx command didn't start");
    }
}
