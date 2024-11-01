// Desenvolva uma função para calcular o fatorial de um número.

fn fatorial(num: i32) -> i32{
    let mut fatorial: i32 = 1;
    for numero in 1..=num{
        fatorial *= numero;
    }

    fatorial
}

fn main(){
    let num = 5;
    println!("O fatorial de {} é: {}", num, fatorial(5));
}