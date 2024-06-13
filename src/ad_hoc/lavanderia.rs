use std::io;

#[allow(dead_code)]
pub fn lavadeira () {
    //Exercicio 1794 
    let mut qtd_roupas:String = String::new();
    let qtd_roupas_int:i16;
    match io::stdin().read_line(&mut qtd_roupas) {
        Ok(_) => {
            qtd_roupas_int = qtd_roupas.trim().parse().expect("erro de parse");
            if qtd_roupas_int < 1 && qtd_roupas_int > 100 {return}
        }
        Err(_) => return
    }
    let mut vetor_size_tot:Vec<[i16;2]> = Vec::new();
    for _ in 0..2 {
        let mut qtd_teste:String = String::new();

        match io::stdin().read_line(&mut qtd_teste) {
            Ok(_) => {
                let qtd_split:Vec<&str> = qtd_teste.split_whitespace().collect();
                let value_1:i16 =  qtd_split[0].trim().parse().expect("msg");
                let value_2:i16 =  qtd_split[1].trim().parse().expect("msg");
                vetor_size_tot.push([value_1,value_2])
            }
            Err(_) => return
        }
    }
    if (qtd_roupas_int >= vetor_size_tot[0][0] && qtd_roupas_int <= vetor_size_tot[0][1]) && 
    (qtd_roupas_int >= vetor_size_tot[1][0] && qtd_roupas_int <= vetor_size_tot[1][1]){
        println!("possivel");
    } 
    else {println!("impossivel")} 
}
