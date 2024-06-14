use std::io;

pub fn top_n() {
    let mut entrada:String = String::new();
    io::stdin().read_line(&mut entrada).expect("msg");
    let entrada_int:i32 = entrada.trim().parse().expect("msg");


    let ranges:[(&str, i32); 7] = [
        ("Top 1", 1),
        ("Top 3", 3),
        ("Top 5", 5),
        ("Top 10", 10),
        ("Top 25", 25),
        ("Top 50", 50),
        ("Top 100", 100),
    ];

    for &(label, limite) in &ranges {
        if entrada_int <= limite {
            println!("{}", label); 
            return;
        }
    }
}