use genetic_algorithm::{data::Problema, genetico::genetico};

fn main() {
    let mut problema = Problema::load_from("instances/ep01.dat");

    let tamanho_populacao: usize = 30;

    let tempo_inicial = std::time::Instant::now();
    for i in 0..30 {
        let (solucao_inicial, melhor_solucao) = genetico(&mut problema, tamanho_populacao);
        let tempo_final = tempo_inicial.elapsed();
        println!("Iteração {i}:");
        println!("- Solução inicial {:?}", solucao_inicial);
        println!("- Solução final {:?}", melhor_solucao);
        println!("- Tempo: {:?}", tempo_final);
    }
}
