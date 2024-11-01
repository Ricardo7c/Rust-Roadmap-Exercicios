// Escreva um programa que exiba a tabuada de um número fornecido pelo usuário.
use std::io::{self, Write};

fn input_number(texto: &str) -> i32{
    loop{
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        // O codigo abaixo converte o que foi digitado para i32 ou exibir a msg de valor invalido.
        match x.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido, Tente novamente!")
        }
    }
}

fn main(){
    let numero = input_number("Digite um numero: ");
    for num in 1..=10{
        println!("{} x {} = {}", numero, num, numero*num);
    }
}