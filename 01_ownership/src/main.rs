fn main() {
    r#f1();
    r#f2();
    r#f3();
    r#f4();
}

fn r#f1() {
    let s1 = String::from("hello");
    let s2 = s1;// <- move: s1 foi movido para s2, e deixou de ser uma referência válida.
    let s3 = s2.clone(); // <- clona tanto o ponteiro na pilha, como o valor no heap

    println!("s2 = {s2}, s3 = {s3}");
}

fn r#f2() {
    let s = String::from("hello");
    let mut x: i32 = 5;

    takes_ownership(s);
    makes_copy(x);

    // println!("x = {s}"); <- invalid -> movido para a função takes_ownership
    println!("x = {x}");
}

fn r#f3() {
    let s1 = gives_ownership();

    println!("s1 = {s1}");

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s3 = {s3}");
}

fn r#f4() {
    let s = String::from("hello");
    let (s2, len) = length_string(s);

    println!("The length of '{}' is {}.", s2, len);
}

fn length_string(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    return "Yours".to_string()
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("some_integer = {some_integer}")
}