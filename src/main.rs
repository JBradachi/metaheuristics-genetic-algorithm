use genetic_algorithm::{
    data::{Problema, Solucao},
    genetico::genetico,
};

fn main() {
    let problema = Problema::load_from("instances/ep01.dat");

    let tamanho_populacao: usize = 10;
    let melhor_solucao: Solucao = genetico(&problema, tamanho_populacao);
    println!("pop inicial {:?}", melhor_solucao);
}
