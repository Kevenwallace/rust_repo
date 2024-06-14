use std::{collections::HashMap, io};

#[allow(dead_code)]
pub fn perdido_marte() {
    //exercicio1986
    let parse:[(&str, &str); 26 ] = [
        ("61", "a"),
        ("62", "b"),
        ("63", "c"),
        ("64", "d"),
        ("65", "e"),
        ("66", "f"),
        ("67", "g"),
        ("68", "h"),
        ("69", "i"),
        ("6A", "j"),
        ("6B", "k"),
        ("6C", "l"),
        ("6D", "m"),
        ("6E", "n"),
        ("6F", "o"),
        ("70", "p"),
        ("71", "q"),
        ("72", "r"),
        ("73", "s"),
        ("74", "t"),
        ("75", "u"),
        ("76", "v"),
        ("77", "w"),
        ("78", "x"),
        ("79", "y"),
        ("7A", "z"),
    ];

    let map: HashMap<_, _> = parse.iter().map(|&(k, v)| (k, v)).collect();

    let mut quantos_caracteres:String = String::new();
    io::stdin().read_line(&mut quantos_caracteres).expect("msg");

    let mut letras_hexa:String = String::new();
    io::stdin().read_line(&mut letras_hexa).expect("msg");
    let letras_hexa_array:Vec<_> = letras_hexa.split_whitespace().collect();

    
    let mut palavra:String = String::new();
    for j in letras_hexa_array {
        if let Some(&charractere) = map.get(j) {
            palavra.push_str(charractere);
        }
    }
    println!("{}", palavra.trim())
}