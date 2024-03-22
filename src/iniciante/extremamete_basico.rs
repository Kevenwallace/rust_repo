use std::io;

pub fn extremamente_basico() {
    //Exercicio 1001
    
    let mut a: String = String::new();
    let mut b: String = String::new();

    
    io::stdin()
        .read_line(&mut a)
        .expect("msg");

    io::stdin()
        .read_line(&mut b)
        .expect("msg");

    let numero_a:i8 = match  a.trim().parse()  {
        Ok(numero) => numero,
        Err(_) => { println!("erro"); return;}
        };

    let numero_b:i8 = match b.trim().parse() {
        Ok(numero) => numero,
        Err(_) => {println!("erro"); return;}
    };

    println!("X = {}", numero_a + numero_b)
}