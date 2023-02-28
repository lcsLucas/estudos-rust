fn main() {
    const phi:f64 = 1.61803;
    const n:f64 = 10.0;

    // WRONG
    let enesimo = (f64::powf(phi, n) - f64::powf((1.0 - phi), n)) / f64::sqrt(5.0);

    println!("{}", enesimo);
    println!("{}", f64::powf(phi, n) - f64::powf((1.0 - phi), n));
}
