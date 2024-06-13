use std::io;

pub fn fatorial () {
//Exercicio 1936
fn fatorial_num(valor:i32) -> i32{
    if valor <= 1 {return valor;}
    return valor * fatorial_num(valor - 1);
}

let mut entrada:String = String::new();
let entrada_int:i32;

match io::stdin().read_line(&mut entrada) {
    Ok(_) => {entrada_int = entrada.trim().parse().expect("erro de converção")},
    Err(_) => {print!("erro"); return;}
}

println!("{}", entrada_int);

let result:i32 = fatorial_num(entrada_int);
println!("{}", result);

let mut count = 1;
loop {
    if entrada_int < fatorial_num(count){
        println!("fazer com o count anterior")
    }
    if entrada_int == fatorial_num(count) {
        println!("finalizado")
    }
    if entrada_int > fatorial_num(count) {
        count +=1;
    }
}

}