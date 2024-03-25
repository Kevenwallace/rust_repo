use std::io;

#[allow(dead_code)]
pub fn diferenca (){
    //Exercicio 1007
    let mut num_a:String = String::new();
    let mut num_b:String = String::new();
    let mut num_c:String = String::new();
    let mut num_d:String = String::new();

    io::stdin().read_line(&mut num_a).expect("error num_a");
    io::stdin().read_line(&mut num_b).expect("error num_b");
    io::stdin().read_line(&mut num_c).expect("error num_c");
    io::stdin().read_line(&mut num_d).expect("error num_d");

    let int_a:i8 = num_a.trim().parse().expect("erro ao fazer o parse para a");
    let int_b:i8 = num_b.trim().parse().expect("erro ao fazer o parse para b");
    let int_c:i8 = num_c.trim().parse().expect("erro ao fazer o parse para c");
    let int_d:i8 = num_d.trim().parse().expect("erro ao fazer o parse para d");

    println!("DIFERENCA = {}", (int_a * int_b) - (int_c * int_d))
}