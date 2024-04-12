use std::io;
use std::collections::HashMap;

pub fn hello_galaxy() {
    //Exercicio 1515
    let mut resultado = Vec::new();
    loop {
        
    let entrada_int:i8;
    let mut entrada:String = String::new();
    match io::stdin().read_line(&mut entrada) {
        Ok(_) => {entrada_int = match entrada.trim().parse() {
            Ok(0) => break,
            Ok(num) => num,
            Err(_) => {println!("error"); return;}}}
        Err(_) => {return;}
    }

    let mut dicionario: HashMap<String, i32> = HashMap::new();

    for _ in 0..entrada_int {
        let mut entrada_dos_planetas:String = String::new();
        match io::stdin().read_line(&mut entrada_dos_planetas) {
            Ok(_) => {
                let separacao_dados:Vec<&str> = entrada_dos_planetas.split_whitespace().collect();
                let parse_data_chegada:i32 = separacao_dados[1].trim().parse().expect("errp");
                let parse_demora_para_chegar:i32 = separacao_dados[2].trim().parse().expect("err");
                let data_inicial:i32 = parse_data_chegada - parse_demora_para_chegar;
                dicionario.insert(String::from(separacao_dados[0]), data_inicial);
                }
            Err(_) =>  {println!("weeoe"); return;}
        }
    }
    if let Some((chave, _)) = dicionario.iter().min_by_key(|&(_, v)| v) {
        resultado.push(chave.clone());
    } else {println!("Dicionário vazio");}
}
    for j in resultado {println!("{}", j)}
}