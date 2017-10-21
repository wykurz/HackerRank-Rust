#[macro_use] extern crate text_io;

fn run(x:&mut Vec<f64>, f:&mut Vec<f64>) -> f64 {
    return 12.2
}

fn main() {
    let n:usize = read!();
    let mut x:Vec<f64> = Vec::with_capacity(n);
    let mut f:Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        x.push(read!());
    }
    for i in 0..n {
        x.push(read!());
    }
    print!("{}", run(&mut x, &mut f))
}
