use genetic_algorithm::{
    data::{Problema},
    genetico::genetico,
};

fn main() {
    let problema = Problema::load_from("instances/ep01.dat");

    let tamanho_populacao: usize = 10;
    let (solucao_inicial, melhor_solucao) = genetico(&problema, tamanho_populacao);
    println!("solução inicial {:?}", solucao_inicial);
    println!("solução final {:?}", melhor_solucao);
}
