fn main() {

    // Tipos de dados:
    // scalar: representa um único valor.

    // integer
    // i: com sinal (possibilidade do número ser negativo)
    //    i8  => integer 8 bits
    //    i32 => integer 32 bits
    //    i64 => integer 64 bits
    // u: sem sinal (números apenas absolutos)
    //    u8  => integer 8 bits
    //    u32 => integer 32 bits
    //    u64 => integer 64 bits

    let guess: u8 = "42".parse().expect("Not a Number!");
    println!("{}", guess);

    // float
    // f32: 32 bits
    // f64: 64 bits

    let x = 2.0; // f64 -> default para variáveis sem o tipo assinado
    let y: f32 = 3.0; // f32

    println!("{}" , x);
    println!("{}" , y);

    // operações numéricas

    println!("sum {} + {} = {}", 5, 10, 5 + 10);
    println!("difference {} - {} = {}", 95.5, 4.3, 95.5 - 4.3);
    println!("product {} * {} = {}", 4, 30, 4 * 30);
    println!("quotient {} / {} = {}", 56.7, 32.2, 56.7 / 32.2);
    println!("truncated {} / {} = {}", -5, 3, -5 / 3);
    println!("truncated_f {} / {} = {}", -5.0, 3.0, -5.0 / 3.0);
    println!("remainder {} % {} = {}", 43, 5, 43 % 5);

    // boolean => bool

    let t = true;
    let f: bool = !t;

    println!("{}", t);
    println!("{}", f);

    // character => char
    let c = 'z';
    let z: char = 'ℤ';
    let e = '😻';

    println!("{}, {}, {}", c, z, e);

    // Tipos de dados:
    // compound: tipo que agrupo vários tipos

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 8);

    let (t1, t2, t3) = tup;

    println!("{}, {}, {}", t1, t2, t3);
    println!("{}", tup.1)

    //array
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 3] = [10, 9, 8];

    let first = a[0];

}
