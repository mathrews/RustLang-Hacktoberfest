
struct A;
struct Single(A);
struct SingleGen<T>(T);

fn main() {
    println!("Hello, world!");
    println!("Eu sou o Mateus, estou experimentando a linguagem Rust!!");

    println!("Legal Matheus, muitos parabéns para você.!!");
    println!("Faça um novo penteado no cabelo. Use cabelo Rustafari!!");
    
    println!("{}", 2 + 2);
    // Teste de Generics.
    let _s = Single(A); // Concreto e explicito
    let _char: SingleGen<char> = SingleGen('a'); //Tipo explicitato pelo parametro
    let _t    = SingleGen(A); // Usa `A` definido no topo.
    let _i32  = SingleGen(6); // Usa `i32`.
    let _char = SingleGen('a'); // Usa `char`.
}

