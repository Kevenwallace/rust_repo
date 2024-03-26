use std::io;

pub fn divisao_nlogonia() {
    //Exercicio 1091
    let mut resultdo: Vec<&str> = Vec::new();
    let mut entrada_int: i16;
    loop {
    let mut entrada:String = String::new();
    
    match io::stdin().read_line(&mut entrada) {
        Ok(0) => break,
        Ok(_) => {entrada_int = match entrada.trim().parse() {Ok(num) => num,Err(_) => {println!("weeoe"); return;}}}
        Err(_) => {println!("error"); return;}
    };
    let mut value_coordinates:String = String::new();
    let mut coordinates_x_y:Vec<i16> = Vec::with_capacity(2);
    match io::stdin().read_line(&mut value_coordinates) {
        Ok(_) => {  let coordinate_split:Vec<&str> = value_coordinates.split_whitespace().collect();
                    for coordinate in coordinate_split 
                    {coordinates_x_y.push(coordinate.trim().parse().expect("error parse"))}}
        Err(_) => {println!("error"); return;}
    };
    let mut vetor_tester:Vec<Vec<i16>> = Vec::new();
    for _ in 0..entrada_int{
        let mut item_vec:String = String::new();
        io::stdin().read_line(&mut  item_vec).expect("error no for");
        let split_item_vec:Vec<&str> = item_vec.split_whitespace().collect();
        let mut int_splited: Vec<i16> = Vec::with_capacity(2);
        for j in split_item_vec{int_splited.push(j.trim().parse().expect("error parse"))}
        vetor_tester.push(int_splited);
    }
    for coordinates_testing in &vetor_tester{
        resultdo.push(if coordinates_testing[0] > coordinates_x_y[0] && coordinates_testing[1] > coordinates_x_y[1] {"NE"}
        else if coordinates_testing[0] < coordinates_x_y[0] && coordinates_testing[1] > coordinates_x_y[1] {"NO"}
        else if coordinates_testing[0] > coordinates_x_y[0] && coordinates_testing[1] < coordinates_x_y[1] {"SE"}
        else if coordinates_testing[0] < coordinates_x_y[0] && coordinates_testing[1] < coordinates_x_y[1] {"SO"} else {"divisa"})}
    }
    for resu in &resultdo {println!("{}",resu );}
}