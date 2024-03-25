use std::io;

#[allow(dead_code)]
pub fn media_2() {
    //Exercicio 1006
    let mut num_a:String = String::new();
    let mut num_b:String = String::new();
    let mut num_c:String = String::new();

    io::stdin()
        .read_line(&mut num_a)
        .expect("err");
    io::stdin()
        .read_line(&mut num_b)
        .expect("err");
    io::stdin()
        .read_line(&mut num_c)
        .expect("err");

    let float_a:f32 = if let Ok(numero) = num_a.trim().parse(){numero} else{println!("erro");return;};
    let float_b:f32 = if let Ok(numero) = num_b.trim().parse(){numero} else{println!("erro");return;};
    let float_c:f32 = if let Ok(numero) = num_c.trim().parse(){numero} else{println!("erro");return;};

    let media = ((float_a * 2f32) + (float_b * 3f32) + (float_c * 5f32)) / 10f32;
    println!("MEDIA = {:.1}", media)


}