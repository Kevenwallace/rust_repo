use std::io;

pub fn media_e_mediana (){
    //Exercicio 1379
    let mut entrada:String = String::new();
    let mut mediana:Vec<i16> = Vec::with_capacity(3);
    io::stdin().read_line(& mut entrada).expect("error");
    let entrada_split:Vec<&str> = entrada.split_whitespace().collect();
    let value_a:i16 = match entrada_split[0].trim().parse() {Ok(num) => num,Err(_) => {println!("error"); return;}};
    let value_b:i16 = match entrada_split[1].trim().parse() {Ok(num) => num,Err(_) => {println!("error"); return;}};

    mediana.push(value_a);
    mediana.push(value_b);

    let tot = ((value_a + value_b) - (value_a * 3))*-1;
    mediana.push(tot);
    mediana.sort();
    println!("mediana 1 = {}", mediana[1]);
    if (value_a + value_b + tot) / 3 == mediana[1] {println!("o valot = {} é valido", tot)}

    
}   