#[macro_use] extern crate text_io;
use std::io::Write;

fn run(x:Vec<i64>, f:Vec<usize>) -> f64 {
    let n:usize = f.iter().sum();
    assert!(n > 0);
    let mut v = Vec::with_capacity(n);
    for (xi, fi) in x.iter().zip(f) {
        for _ in 0..fi {
            v.push(xi);
        }
    }
    v.sort();
    let hsize = n / 2;
    if hsize & 1 == 1 {
        let i = hsize / 2;
        let q1 = v[i];
        let q3 = v[n - 1 - i];
        writeln!(std::io::stderr(), "A: i: {}, q1: {}, q3: {}", i, q1, q3);
        return (q3 - q1) as f64;
    }
    assert!(hsize > 1);
    let i = hsize / 2 - 1;
    let q1 = ((v[i] + v[i + 1]) as f64) / 2.0;
    let q3 = ((v[n - 1 - i] + v[n - 2 - i]) as f64) / 2.0;
    writeln!(std::io::stderr(), "B: i: {}, q1: {}, q3: {}", i, q1, q3);
    return q3 - q1
}

fn main() {
    let n:usize = read!();
    let mut x:Vec<i64> = Vec::with_capacity(n);
    let mut f:Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        x.push(read!());
    }
    for _ in 0..n {
        f.push(read!());
    }
    println!("{}", format!("{:.*}", 1, run(x, f)))
}
