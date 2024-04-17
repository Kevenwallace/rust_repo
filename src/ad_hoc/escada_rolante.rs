use std::io;

pub fn escada_rolante (){
    //Exercicio 1793
    let mut resultado:Vec<i16> = Vec::new();
    loop {
        let qtd_pessoas_int:i16;
        let mut qtd_pessoas :String = String::new();
        let mut vec_tempos_de_entrada_int:Vec<i16> = Vec::new();
        match io::stdin().read_line(&mut qtd_pessoas) {
            Ok(_) => {
                qtd_pessoas_int = qtd_pessoas.trim().parse().expect("err");
                if qtd_pessoas_int == 0 {break;}
            }
            Err(_) => return
        }
    let mut tempo_de_entrada:String = String::new();
    let vec_tempos_de_entrada:Vec<&str>;
    match io::stdin().read_line(&mut tempo_de_entrada) {
        Ok (_) => {
            vec_tempos_de_entrada = tempo_de_entrada.split_whitespace().collect();
            for tempo_entrada in vec_tempos_de_entrada {
                match tempo_entrada.trim().parse::<i16>() {
                    Ok(num) => {vec_tempos_de_entrada_int.push(num)}
                    Err(_) => return
                }
            }
        }
        Err(_) => return
    }
    let mut contagem:i16 = 0;
    for index in 0..vec_tempos_de_entrada_int.len(){
        if index + 1 < vec_tempos_de_entrada_int.len() {
            if (vec_tempos_de_entrada_int[index] + 10) >= vec_tempos_de_entrada_int[index + 1] {
                contagem += 10 - ((vec_tempos_de_entrada_int[index] + 10) - vec_tempos_de_entrada_int[index + 1]);
            } else {contagem += 10;}
        } else {contagem += 10;}
    }
    resultado.push(contagem)
}
for contagem in resultado {println!("{}", contagem)}
}