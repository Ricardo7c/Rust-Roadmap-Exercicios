// Implemente um programa que peça ao usuário para inserir um número e verifique se ele é par ou ímpar.
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
    let valor = input_number("Digite um numero: ");
    if valor % 2 == 0{
        println!("{} é Par", valor);
    }else{
        println!("{} é Impar", valor);
    }
}

