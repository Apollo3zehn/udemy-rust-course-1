use num_traits::ToPrimitive;

fn solve<T1: ToPrimitive, T2: ToPrimitive>(a: T1, b: T2) -> f64 {

    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: i8 = 3;
    let b: f64 = 4.0;

    println!("{}", solve(a, b));
}