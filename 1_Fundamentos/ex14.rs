// Implemente um programa que conte de 1 a 50 e imprima "Fizz" para múltiplos de 3 e "Buzz" para múltiplos de 5.

fn main(){
    for num in 1..=50{
        if num % 3 == 0 && num % 5 == 0{
            println!("FizzBuzz");
        }else if num % 3 == 0 {
            println!("Fizz");
        }else if num % 5 == 0{
            println!("Buzz");
        }else{
            println!("{}", num);
        }
    }
}