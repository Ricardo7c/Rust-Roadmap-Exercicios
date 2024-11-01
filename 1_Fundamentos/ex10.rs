// Crie um programa que gere uma sequência de Fibonacci até o N-ésimo número, fornecido pelo usuário.

use std::io::{self, Write};

fn input_number(texto: &str) -> u128{
    loop{
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        // O codigo abaixo converte o que foi digitado para i32 ou exibir a msg de valor invalido.
        match x.trim().parse::<u128>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido, Tente novamente!")
        }
    }
}

fn main(){
    let num:u128 = input_number("Digite um numero: ");
    let mut a:u128 = 1;
    let mut b:u128 = 1;
    let mut fib= vec![1];

    for _ in 1..num{
        fib.push(b);
        (a, b) = (b, a+b);
    }
    print!("{:?}", fib);
}