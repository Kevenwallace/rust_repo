use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn media_e_mediana (){
    //Exercicio 1379
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut valores_totais: Vec<Vec<i64>> = Vec::new();

    for lines in handle.lines(){
        if let Ok(entrada) = lines {
            let valores :Vec<i64> = entrada
            .trim()
            .split_whitespace()
            .map(|val|val.parse::<i64>())
            .filter_map(Result::ok)
            .collect();

            if valores.len() < 2 {
                println!("Erro: Pelo menos dois valores inteiros são necessários");
                return;
            }

            let value_a:Option<i64> = Some(valores[0]);
            let value_b:Option<i64> = Some(valores[1]);

            if let (Some(a), Some(b)) = (value_a, value_b) {
                if a == 0 && b == 0 {break;}
                let new_vec: Vec<i64> = vec![a, b];
                valores_totais.push(new_vec);
                }
        }
    }
    let mut array_result:Vec<i64> = Vec::new();
    for valor_unico in valores_totais{
        let mut x_value:Vec<i64> = Vec::with_capacity(2);
        for i in &valor_unico {
                let tot = ((valor_unico[0] + &valor_unico[1]) - (i * 3))*-1; x_value.push(tot);
        }
        x_value.sort();
        array_result.push(x_value[0])
    }
    for result in array_result {println!("{}", result)}
} 