// Desenvolva um programa que calcule a área e o perímetro de um círculo, recebendo o raio como entrada.
const PI: f64 = 3.14;
fn main(){
    let raio = 5 as f64;
    let area = PI*(raio*raio);
    let perimetro = (PI*raio)* 2 as f64;

    println!("Um circulo com o raio de {:.2}, tem uma area de {:.2} e um perimetro de {:.2}", raio, area, perimetro);
}