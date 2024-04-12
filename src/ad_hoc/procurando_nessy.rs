use std::io;
#[allow(dead_code)]
pub fn procurando_nessy() {
    // Exercicio 1428
    let mut numero_entradas:String = String::new();
    let mut datagrama:Vec<i32> = Vec::new();

    io::stdin().read_line(&mut numero_entradas).expect("erro");
    let numero_entradas_int:i32 = match numero_entradas.trim().parse() {Ok(num) => num,Err(_) => {println!("erro so converter");return;}};
    
    for _ in 0..numero_entradas_int {
        let mut entrada_valor:String = String::new();
        let mut vetor_data_grama: Vec<i32> = Vec::new();

        io::stdin().read_line(&mut entrada_valor).expect("erro");

        let vetor_entrada:Vec<&str> = entrada_valor.split_whitespace().collect();

        for value in vetor_entrada {
            let vetor_item:i32 = match value.trim().parse() {Ok(num) => num,Err(_) => {println!("erro so converter");return;}};
            vetor_data_grama.push(vetor_item)
        }

        let resultado = (vetor_data_grama[0]/3) * (vetor_data_grama[1]/3);
        datagrama.push(resultado)
    }
    for j in datagrama {println!("{}", j)}
}