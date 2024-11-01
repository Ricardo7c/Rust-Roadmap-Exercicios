// Crie um jogo de adivinhação onde o programa escolhe um número aleatório e o usuário tenta adivinhar.
use std::io::{self, Write};
use rand::Rng;

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
    let aleatorio: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Pensei em um numero, você consegue adivinhar qual foi? ");
    let numero = input_number("Digite o seu palpite: ");
    if numero == aleatorio{
        println!("Você acertou!");
    }else{
        println!("ERROU! o numero que pensei foi: {}", aleatorio);
    }
}