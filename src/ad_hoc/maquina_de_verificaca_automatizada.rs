use std::io;


pub fn maquina_de_verificaca_automatizada() {
    // Exercicio 1743
    let mut entrada_1:String = String::new();
    let mut entrada_2:String = String::new();

    let entrada_1_split:Vec<&str>;
    let entrada_2_split:Vec<&str>;

    match io::stdin().read_line(&mut entrada_1) {
        Ok(_) => {entrada_1_split = entrada_1.trim().split_whitespace().collect();}
        Err(_) => {return;}
    }
    match io::stdin().read_line(&mut entrada_2) {
        Ok(_) => {entrada_2_split = entrada_2.trim().split_whitespace().collect();}
        Err(_) => {return;}
    }
    for pino in 0..=4 {
        if entrada_1_split[pino] == entrada_2_split[pino] {println!("N"); return;}
    }
    println!("Y")
}