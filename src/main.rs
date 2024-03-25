mod iniciante {
    pub mod  hello_world;
    pub mod extremamete_basico;
    pub mod area_do_circulo;
    pub mod soma_simples;
    pub mod produto_simples;
    pub mod media_1;
    pub mod media_2;
}

#[allow(unused_imports)]
use iniciante::{
    extremamete_basico::extremamente_basico,
    hello_world::funcao,
    area_do_circulo::area_do_circulo,
    soma_simples::soma_simples,
    produto_simples::produto_simples,
    media_1::media1,
    media_2::media_2
};

fn main() {
    // funcao();
    // extremamente_basico();
    // area_do_circulo();
    // produto_simples();
    // media1()
    media_2()
}
