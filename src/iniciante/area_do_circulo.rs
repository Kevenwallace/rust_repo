use std::io;

static PI:f64 = 3.14159;

pub fn area_do_circulo (){

    let mut raio: String = String::new();

    io::stdin()
        .read_line(&mut raio)
        .expect("error");

    // let raio_parse:f64 = match raio.trim().parse() {
    //     Ok(numero) => numero,
    //     Err(_) => {println!("erro"); return;}
    // };

    
    if let Ok(numero_raio) = raio.trim().parse::<f64>()
    {
        println!("A={:.4}", numero_raio.powi(2) * PI);
    };

    // println!("A={:.4}", numero_raio.powi(2) * PI)
}