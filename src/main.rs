use genetic_algorithm::data::{solucao::populacao_inicial, Problema, Solucao};

fn main() {
    let problema = Problema::load_from("instances/ep01.dat");

    let pressao = 1.0;
    let pop_inicial: Vec<Solucao> = populacao_inicial(10, pressao, &problema);

    println!("pop inicial {:?}", pop_inicial);

}
