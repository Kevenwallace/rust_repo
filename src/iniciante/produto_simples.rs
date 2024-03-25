use std::io;

#[allow(dead_code)]
pub fn produto_simples() {
    //Exercicio 1004

    let mut num_a:String = String::new();
    let mut num_b:String = String::new();

    io::stdin()
        .read_line(&mut num_a)
        .expect("error");

    io::stdin()
        .read_line(&mut num_b)
        .expect("error");

    let numero_a:i16 = if let Ok(numero) = num_a.trim().parse(){numero} else {println!("error"); return;};
    let numero_b:i16 = if let Ok(numero) = num_b.trim().parse(){numero} else {println!("error"); return;};

    println!("PROD = {}", numero_a * numero_b)
}