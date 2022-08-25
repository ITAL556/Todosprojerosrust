use std::io;
use std::f64::consts::PI;
pub fn circulo() {
    let mut width1 = String::new();
    let pi = std::f64::consts::PI;
    println!("Por favor digite as medidas do retangulo");
    io::stdin().read_line(&mut width1).expect("Não foi possivel gardar numero");   
    let larg = width1.trim().parse::<f64>().expect("converçao deu erro");
    println!("Com as medidas fornecidas a area do seu retangulo é:{}", area(larg,pi));
}
fn area(larg: f64, pi: f64) -> f64 {
    (larg * larg) * pi
}