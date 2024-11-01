// Escreva uma função que receba dois números inteiros e retorne a soma deles.

fn soma(n1: i32, n2:i32) -> i32{
    n1+n2
}

fn main(){
    let n1 = 2;
    let n2 = 3;
    println!("{n1} + {n2} = {}", soma(n1, n2))
}