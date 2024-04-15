use std::io;

#[allow(dead_code)]
pub fn feed_back() {
    //Exercicio 1546
    let qtd_feedbacks_int:i32;
    let mut qtd_feedbacks:String = String::new();
    let mut resultado:Vec<&str> = Vec::new();
    match io::stdin().read_line(&mut qtd_feedbacks) {
        Ok(_) => {qtd_feedbacks_int = qtd_feedbacks.trim().parse().expect("erro")}
        Err(_) => {println!("erro ao pegar os valores");return;}
    }

    for _ in 0..qtd_feedbacks_int {
        let qtd_por_feedbacks_int:i32;
        let mut qtd_por_feedbacks:String = String::new();

        match io::stdin().read_line(&mut qtd_por_feedbacks) {
            Ok(_) => {qtd_por_feedbacks_int = qtd_por_feedbacks.trim().parse().expect("erro")}
            Err(_) => {println!("erro ao pegar os valores");return;}
        }

        for _ in 0..qtd_por_feedbacks_int {
            let mut teste:String = String::new();
            match io::stdin().read_line(&mut teste) {
                Ok(_) => {let result:&str = match teste.trim() {
                    "1" => "Rolien",
                    "2" => "Naej",
                    "3" => "Elehcim",
                    "4" => "Odranoel",
                    _ => "error"
                };
                resultado.push(result)
                }
                Err(_) => {println!("erro ao pegar os valores");return;}
            }
        }
    }
    for value in resultado {println!("{}", value)}
}