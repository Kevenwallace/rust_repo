use std::io;

pub fn extremamente_basico() {
    //Exercicio 1001
    
    let mut a= String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("msg");

    let numero_a:i8 = match  a.trim().parse()  {
        Ok(numero) => numero,
        Err(_) => { println!("erro"); return;}
        };

    println!("{}", numero_a)
}