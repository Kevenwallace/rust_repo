use std::io;

pub fn jogo_do_maior() {

    let mut qtd_entrada_int:i8;
    loop {
        let mut qtd_entrada:String = String::new();
        
        match io::stdin().read_line(& mut qtd_entrada) {
            Ok(0) => break,
            Ok(_) => {qtd_entrada_int = match qtd_entrada.trim().parse() {Ok(num) => num, Err(_)=> {println!("erro"); return;}}},
            Err(_) => {println!("erro"); return;}
            
        }
    
        for _ in 0..qtd_entrada_int{
            let entrada_jogo:String = String::new();
        }
    }
}