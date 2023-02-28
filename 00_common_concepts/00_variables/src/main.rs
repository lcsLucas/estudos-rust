fn main() {

    // Variáveis podem ser mutáveis ou imutáveis:
    //      imutáveis (let): Variáveis que não podem ter seu valor alterados, após a definição
    //      mutáveis (let mut): Variáveis que podem ter seu valor alterados no decorrer do código

    let _y = 10; // variável imutável
    let mut x = 5; // variável mutável
    
    println!("The value of 'x' is: {}", x);
    x = 6;
    println!("The new value of 'x' is: {}", x);

    // Contantes (const): são SEMPRE imutáveis e devem ter seu tipo e valores definidos na declaração; 
    // além disso não pode utilizar variáveis para a definição de seu valor.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The 'THREE_HOURS_IN_SECONDS' value is: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing: Essa expressão se refere quando uma segunda variável é criada (redeclaração de uma variável) acontece,
    // a primeira é ofuscada pela segunda, então se na primeira tivéssemos uma declaração u32, e na segunda declaração uma string,
    // então o programa deixaria de ver a variável como u32 para agora trata-lá como string.

    {
        let _y = _y * 2; // Shadowing acontecendo aqui.
        println!("The value of '_y' in the inner scope is: {}", _y);
    }

    println!("test value of '_y' is: {}", _y);

}
