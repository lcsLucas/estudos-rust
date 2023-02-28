//
// statements vs expressions
//  - statements: são instruções que executam uma ação sem retorno de valor. 
//      ex: let x = 6;
//  - expression: valor resultante
//      ex: x + 1

fn main() {
    println!("Hello World!");
    another_function("Lucas", 28);

    let ex = {
        let x = 3; // statement -> não possui retorno
        let y = 0;
        x + 1;
        y - 1 // expression -> não incluem ponto-e-vírgula final
    };

    // let _five = five();
    let _six = plus_one(five().into());

    // println!("fn_another value is {fn_another}");
    println!("ex value is {ex}");
    println!("five + plus_one value is {_six}");
}

fn another_function(name: &str, age: u8) {
    println!("Your name is => {name} and your age is {age}");
}

fn five() -> i8 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}