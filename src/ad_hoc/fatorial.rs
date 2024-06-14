use std::io;

#[allow(dead_code)]
pub fn fatorial () {
//Exercicio 1936
fn fatorial_num(valor:i32) -> i32{
    if valor <= 1 {return 1}
    return valor * fatorial_num(valor - 1);
}
let mut entrada:String = String::new();
let mut entrada_int:i32;
io::stdin().read_line(&mut entrada).expect("error");
entrada_int = entrada.trim().parse().expect("erro de converção");
let mut count = 1;
let mut quantidade:i32 = 0;
while entrada_int > 0 {
    let valor_fatorial:i32 = fatorial_num(count);
    if entrada_int < valor_fatorial{
        entrada_int = entrada_int - fatorial_num(count -1);
        quantidade +=1; count = 1;
    } else if entrada_int == valor_fatorial {quantidade += 1; break;
    } else {count +=1;}
}
println!("{}", quantidade)
}