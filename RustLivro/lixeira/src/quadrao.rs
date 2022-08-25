use std::io;
pub fn quadrado() {
    let mut width1 = String::new();
    let mut height1 = String::new();
    println!("Por favor digite as medidas do retangulo");
    io::stdin().read_line(&mut width1).expect("Não foi possivel gardar numero");
    io::stdin().read_line(&mut height1).expect("Não foi possivel gardar numero");   
    let larg = width1.trim().parse::<f32>().expect("converçao deu erro");
    let compr = height1.trim().parse::<f32>().expect("converçao deu erro");
    println!("Com as medidas fornecidas a area do seu retangulo é:{}", area(larg,compr));
}
fn area(larg: f32, compr: f32) -> f32 {
    larg * compr
}