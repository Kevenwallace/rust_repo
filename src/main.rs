mod iniciante {
    pub mod  hello_world;
    pub mod extremamete_basico;
}

use iniciante::{extremamete_basico::extremamente_basico, hello_world::funcao};

fn main() {
   funcao();
   extremamente_basico()
}
