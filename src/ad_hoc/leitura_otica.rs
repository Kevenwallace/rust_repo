use std::io;

#[allow(dead_code)]
pub fn leitura_otica(){
    //Exercicio 1129
    let mut entrada_int:i8;
    let mut texto:Vec<char> = Vec::new();
    loop {
        let mut entrada:String = String::new();

        match io::stdin().read_line(&mut entrada) {
            Ok(_) => {entrada_int = match entrada.trim().parse() {
                Ok(0) => break,
                Ok(num) => num,
                Err(_) => {println!("error"); return;}}},
            Err(_) => {println!("error"); return;}
        };
        
        let mut array_de_notas: Vec<Vec<i16>> = Vec::new();
        for _ in 0..entrada_int {
            let mut cor_notas:Vec<i16>= Vec::with_capacity(4);
            let mut valores:String = String::new();
            
            match io::stdin().read_line(&mut valores) {
                Ok(_) => {let valores_sepados: Vec<&str> = valores.split_whitespace().collect();
                        for valores in valores_sepados {cor_notas.push(valores.trim().parse().expect("err"))}
                        array_de_notas.push(cor_notas);},
                Err(_) => {println!("error"); return;}}
        }
        let mut count:i8;
        let mut chat:char;
        for notas in &array_de_notas {
            count = 1;
            chat  = ' ';
            for nota in notas {
                if *nota <= 127 {
                    if chat.is_whitespace() {
                        chat =  match count {
                            1 => 'A',
                            2 => 'B',
                            3 => 'C',
                            4 => 'D',
                            5 => 'E',
                            _ => '*'};
                            texto.push(chat);
                            continue;
                    }
                    texto.pop();
                    texto.push('*');
                    continue;
                }
                if chat.is_whitespace() && count == 5 {texto.push('*')}
                count+=1;
            }
        }
    };
    for i in &texto {
        println!("{}", i)
    }

}