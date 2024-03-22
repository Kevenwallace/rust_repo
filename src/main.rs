mod iniciante {
    pub mod  hello_world;
    pub mod extremamete_basico;
    pub mod area_do_circulo;
    pub mod soma_simples;
    pub mod produto_simples;
}

#[allow(dead_code)]
use iniciante::{
    extremamete_basico::extremamente_basico,
    hello_world::funcao,
    area_do_circulo::area_do_circulo,
    soma_simples::soma_simples,
    produto_simples::produto_simples
};

fn main() {
    // funcao();
    // extremamente_basico();
    // area_do_circulo();
    produto_simples();
}
