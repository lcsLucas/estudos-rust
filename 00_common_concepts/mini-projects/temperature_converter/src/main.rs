// Converter entre temperaturas.
//p
//1 - 1
//
//i

fn main() {
    let c: f32 = 0.0;
    let f: f32 = 32.0;

    let c2 = r#fahrenheit_2_celsius(f);
    let f2 = r#celsius_2_fahrenheit(c);

    println!("{f} ºF to {c2} ºC");
    println!("{c} ºC to {f2} ºF");
}

fn r#fahrenheit_2_celsius(f: f32) -> f32 {
    return 5.0 / 9.0 * (f - 32.0);
}

fn r#celsius_2_fahrenheit(c: f32) -> f32 {
    return (c * 9.0 / 5.0) + 32.0;
}