mod iniciante  {
    pub mod hello_world;
    pub mod extremamete_basico;
    pub mod area_do_circulo;
    pub mod soma_simples;
    pub mod produto_simples;
    pub mod media_1;
    pub mod media_2;
    pub mod diferenca;
}



#[allow(unused_imports)]
use iniciante::{
    extremamete_basico::extremamente_basico,
    hello_world::funcao,
    area_do_circulo::area_do_circulo,
    soma_simples::soma_simples,
    produto_simples::produto_simples,
    media_1::media1,
    media_2::media_2,
    diferenca::diferenca
};
mod ad_hoc {
    pub mod divisao_nlogonia;
    pub mod leitura_otica;
    pub mod media_e_mediana;
}

#[allow(unused_imports)]
use ad_hoc::{
    divisao_nlogonia::divisao_nlogonia,
    leitura_otica::leitura_otica,
    media_e_mediana::media_e_mediana
};

fn main() {
    // leitura_otica()
    // divisao_nlogonia()
    media_e_mediana()
}
