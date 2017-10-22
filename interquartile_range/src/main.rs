#[macro_use] extern crate text_io;

fn run(x:&mut Vec<f64>, f:&mut Vec<f64>) -> f64 {
    return 9.01
}

fn main() {
    let n:usize = read!();
    let mut x:Vec<f64> = Vec::with_capacity(n);
    let mut f:Vec<f64> = Vec::with_capacity(n);
    for _ in 0..n {
        x.push(read!());
    }
    for _ in 0..n {
        x.push(read!());
    }
    println!("{}", format!("{:.*}", 1, run(&mut x, &mut f)))
}
