// Escreva uma função que verifique se um número é positivo, negativo ou zero.

fn verifica(n: i32){
    if n > 0{
        println!("{} é Positivo", n);
    }else if n < 0{
        println!("{} é Negativo", n);
    }else{
        println!("O numero é Zero!");
    }
}

fn main(){
    let n = -9;
    verifica(n);
}
