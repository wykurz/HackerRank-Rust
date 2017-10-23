use std::io::prelude::*;
use std::process::{Command, Stdio};

fn test_output(input:&str, output:&str) {
    println!(concat!("target/debug/", env!("CARGO_PKG_NAME")));
    let process = Command::new(concat!("target/debug/", env!("CARGO_PKG_NAME")))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    process.stdin.unwrap().write_all(input.as_bytes()).expect("Failed writing to stdin");

    let mut res = String::new();
    process.stdout.unwrap().read_to_string(&mut res).expect("Failed reading from to stdout");
    assert_eq!(output, res);
}

#[test]
fn test1() {
    test_output("6\n6 12 8 10 20 16\n5 4 3 2 1 5", "9.0\n");
}

#[test]
fn test2() {
    test_output("7\n1 2 3 4 5 6 7\n1 1 1 1 1 1 1", "5.0\n");
}

#[test]
fn test3() {
    test_output("8\n1 2 3 4 5 6 7 8\n1 1 1 1 1 1 1", "4.0\n");
}
