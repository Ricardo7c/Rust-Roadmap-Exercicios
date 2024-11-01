// Crie um programa que converta quilômetros em milhas, usando uma constante de conversão.
const CONVERSAO:f64 = 1.609;

fn main(){
    let km = 200 as f64;
    let milhas = km/CONVERSAO;
    println!("{} km equivalem a {:.2} Milhas", km, milhas);
}