use std::io;

#[allow(dead_code)]
pub fn media1(){
    //Exercicio 1005
    let mut number_a:String = String::new();
    let mut number_b:String = String::new();

    io::stdin()
        .read_line(&mut number_a)
        .expect("error");

    io::stdin()
        .read_line(&mut number_b)
        .expect("error");

    let float_a:f32 = if let Ok(number) = number_a.trim().parse() {number} else {println!("error"); return;};
    let float_b:f32 = if let Ok(number) = number_b.trim().parse() {number} else {println!("error"); return;};

    let result:f32 = ((float_a * 3.5) + (float_b * 7.5) ) / 11f32;
    println!("MEDIA = {:.5}", result)
}