use std::io;

pub fn divisao_nlogonia() {
    //Exercicio 1091
    //https://judge.beecrowd.com/pt/problems/view/1091
    let mut kuantity:String = String::new();
    io::stdin().read_line(&mut kuantity).expect("error");
    let kuantity_int:i16 = kuantity.trim().parse().expect("error parse");
    
    let mut value_cordenade:String = String::new();
    io::stdin().read_line(&mut value_cordenade).expect("error");
    
    let corden_split:Vec<&str> = value_cordenade.split_whitespace().collect();
    println!("{:?}", corden_split);

    let mut vetor:Vec<Vec<i16>> = Vec::new();
    for _ in 0..kuantity_int{
        let mut item_vec:String = String::new();
        io::stdin().read_line(&mut  item_vec).expect("error no for");
        let split_item_vec:Vec<&str> = item_vec.split_whitespace().collect();
        
        let mut int_splited: Vec<i16> = Vec::with_capacity(2);
        for j in split_item_vec{
            
            int_splited.push(j.trim().parse().expect("error parse"))
        }
        vetor.push(int_splited);
    }

    println!("{:?}",vetor)

}