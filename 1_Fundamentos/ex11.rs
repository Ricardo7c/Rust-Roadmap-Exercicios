// Crie uma função que receba dois números e retorne o maior entre eles.

fn maior(n1: i32, n2: i32) -> i32{
    if n1 > n2{
        n1
    }else{
        n2
    }
}

fn main(){
    let n1 = 2;
    let n2 = 8;
    println!("O maior numero entre {} e {} é: {}", n1, n2, maior(n1, n2));
}