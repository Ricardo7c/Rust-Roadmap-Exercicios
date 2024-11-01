// Faça um programa que calcule a média de três notas inseridas e exiba a mensagem de aprovação ou reprovação.
use std::io::{self, Write};

fn input_number(texto: &str) -> f64{
    loop{
        print!("{}",texto);
        io::stdout().flush().unwrap();
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        // O codigo abaixo converte o que foi digitado para i32 ou exibir a msg de valor invalido.
        match x.trim().parse::<f64>() {
            Ok(x) => return x,
            Err(_) => println!("Valor invalido, Tente novamente!")
        }
    }
}

fn main(){
    let nota1 = input_number("Digite a primeira nota: ");
    let nota2 = input_number("Digite a segunda nota: ");
    let nota3 = input_number("Digite a terceira nota: ");

    let media = (nota1+nota2+nota3)/3 as f64;
    if media >= 7 as f64{
        println!("Aprovado com media: {:.1}", media);
    }else {
        println!("Reprovado com media: {:.1}", media);
    }
}