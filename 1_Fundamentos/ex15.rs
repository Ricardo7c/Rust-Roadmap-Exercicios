// Crie uma função que receba uma string e retorne-a em letras maiúsculas.

fn maiscula(palavra: String) -> String{
    palavra.to_uppercase()
}

fn main(){
    let palavra = "Ricardo".to_string();
    println!("{}", maiscula(palavra));
}