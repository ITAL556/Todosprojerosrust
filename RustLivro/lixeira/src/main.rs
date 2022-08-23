use core::num;
use std::io;

mod esfera;
mod quadrao;

fn main() 
{

    loop
    {
        println!("Qual tipo de poligino quer calcular\n 1 = Retangulo\n2 = Esfera(circulo)");
      
    
        let mut calc_poli = String::new();
        
        io::stdin()
        .read_line(&mut calc_poli)
        .expect("Failed to read line");

        if calc_poli.trim() == "1"
        {
            println!("Você selecionou ""Retangulo"" agora vou precisar que vc informe as medidas do retangulo");
            quadrao::quadrado();
            break;
        }

       else if calc_poli.trim() == "2"
       {
            println!("o codigo que ira rodar é ma.rs");
            esfera::circulo();
            break;
       }

       else if calc_poli.trim() == "exit"
       {
        break;
       }
       
       else
       { 
            println!("nenhum codigo valido por favor inserir novamente nome correto")
       }
       
    }
                
        
}

