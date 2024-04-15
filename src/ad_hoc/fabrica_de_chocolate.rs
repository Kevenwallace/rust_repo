use std::io;

pub fn fabrica_de_chocolate () {
    //Exercicio 1573
    let mut aresta_cubo: Vec<i64> = Vec::new();
    loop {
        let mut value_int:Vec<f64> = Vec::new();
        let mut entrada:String = String::new();
        match io::stdin().read_line(&mut entrada) {
            Ok(_) => {  
                let entrada_slit:Vec<&str> = entrada.trim().split_whitespace().collect();
                for value in entrada_slit {
                    match value.parse::<f64>() {
                        Ok(num) => {value_int.push(num)}
                        Err(_) => {return;}
                    }
                }
            }
            Err(_) => {return;}
        }
        if value_int[0] == 0.0 && value_int[1] == 0.0 && value_int[2] == 0.0 {break;}
        let result:f64 = (value_int[0] * value_int[1] * value_int[2]).cbrt() ;       
        aresta_cubo.push(result.floor() as i64)
    }
    for arestas in aresta_cubo {println!("{}", arestas)}
}