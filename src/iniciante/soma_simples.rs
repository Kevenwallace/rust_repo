use std::io;

#[allow(dead_code)]
pub fn soma_simples(){
    //Exerceicio 1003

    let mut a:String = String::new();
    let mut b:String = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("error");

    io::stdin()
        .read_line(&mut b)
        .expect("err");

    //usando if let
    let numero_a:i8 = if let Ok(numero) = a.trim().parse() {numero} else {println!("error"); return;};
    //usando match
    let numero_b:i8 = match b.trim().parse() {Ok(numero)=> numero, Err(_) => {println!("erro"); return;}};
    println!("SOMA = {}", numero_a + numero_b);
    
}