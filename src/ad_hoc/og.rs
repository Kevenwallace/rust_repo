use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn og() {
    //Exercicio 1387 
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut entrada_geral: Vec<Vec<i16>> = Vec::new();

    for line in handle.lines() {
        
        if let Ok(entrada) = line {
            let valores:Vec<i16> = entrada.trim()
            .split_whitespace()
            .map(|val| val.parse::<i16>())
            .filter_map(Result::ok)
            .collect();
        
            let left:i16 = valores[0];
            let rigth:i16 = valores[1];
            if (left == 0) && (rigth == 0) {break;}
            entrada_geral.push(valores)
        }
    }
    for filhos in entrada_geral {println!("{}", filhos[0] + filhos[1])}
}