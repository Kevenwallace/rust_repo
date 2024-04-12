use std::io;

#[allow(dead_code)]
pub fn jogo_do_maior() {
    // exercicio 1397
    let mut qtd_entrada_int:i8;
    let mut pontos:Vec<[i8;2]> = Vec::new();
    loop {
        let mut qtd_entrada:String = String::new();
        match io::stdin().read_line(& mut qtd_entrada) {
            Ok(_) => {qtd_entrada_int = match qtd_entrada.trim().parse() {
                Ok(0) => break,
                Ok(num) => num,
                Err(_)=> {println!("erro"); return;}}},
            Err(_) => {println!("erro"); return;}
        }
        let mut pontos_left:i8 = 0;
        let mut pontos_rigth:i8 = 0;
    
        for _ in 0..qtd_entrada_int{
            let mut entrada_jogo:String = String::new();
            
            io::stdin()
                .read_line(&mut entrada_jogo)
                .expect("err");
            
            let entrada_split: Vec<&str> = entrada_jogo.split_whitespace().collect();
            let left:i64 = match entrada_split[0].trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("erro"); return;}
            };
            let rigth:i64 = match entrada_split[1].trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("erro"); return;}
            };
            if left > rigth {pontos_left += 1;}
            if rigth > left {pontos_rigth += 1;}
        }
        pontos.push([pontos_left, pontos_rigth])    
    }
    for ponto in pontos {println!("{} {}", ponto[0], ponto[1])}
}