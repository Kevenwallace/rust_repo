use std::io;

pub fn divisao_nlogonia() {
    //Exercicio 1091
    //https://judge.beecrowd.com/pt/problems/view/1091
    let mut resultdo: Vec<&str> = Vec::new();

    loop {
    let mut kuantity:String = String::new();
    io::stdin().read_line(&mut kuantity).expect("error");
    let kuantity_int:i16 = kuantity.trim().parse().expect("error parse");
    if kuantity_int == 0{break;}
    let mut value_cordenade:String = String::new();
    io::stdin().read_line(&mut value_cordenade).expect("error");
    
    let corden_split:Vec<&str> = value_cordenade.split_whitespace().collect();
    let mut corden_split_int:Vec<i16> = Vec::with_capacity(2);
    for l in corden_split {corden_split_int.push(l.trim().parse().expect("error parse"))}


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
    for k in &vetor{
        resultdo.push(if k[0] > corden_split_int[0] && k[1] > corden_split_int[1] {"NE"}
         else if k[0] < corden_split_int[0] && k[1] > corden_split_int[1] {"NO"}
         else if k[0] > corden_split_int[0] && k[1] < corden_split_int[1] {"SE"}
         else if k[0] < corden_split_int[0] && k[1] < corden_split_int[1] {"SO"} else {"divisa"})
    }

    }
    for resu in &resultdo {
        println!("{}",resu );
    }
}