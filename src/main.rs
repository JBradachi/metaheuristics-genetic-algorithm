use genetic_algorithm::{
    data::Problema,
    genetico::genetico,
};

fn main() {
    let mut problema = Problema::load_from("instances/ep01.dat");

    let tamanho_populacao: usize = 30;
    let (solucao_inicial, melhor_solucao) = genetico(&mut problema, tamanho_populacao);
    println!("solução inicial {:?}", solucao_inicial);
    println!("solução final {:?}", melhor_solucao);
}
