use std::process::Command;
use std::{thread, time};
use std::env;

fn getenv( key: &str ) -> String {
    match env::var( &key ) { 
        Ok( value ) => {
            format!(
                "{v}",
                v = &value
            )   
        },  
        Err( error ) => {
            format!( "{key}, {error}", key = &key, error = error )
        },  
    }   
}

fn main() {
    let mut cmd = Command::new("nginx");
    let ttl: u64 = getenv("TTL").parse().unwrap();

    println!("TTL: {}sec", ttl);


    if let Ok(mut child) = cmd.args(&["-g", "daemon off;"]).spawn() {
        println!("start nginx");
        let day = time::Duration::from_secs(ttl);
        let now = time::Instant::now();

        thread::sleep(day);
        assert!(now.elapsed() >= day);

        child.kill().expect("command wasn't running");
        println!("stop nginx");
    } else {
        println!("nginx command didn't start");
    }
}
