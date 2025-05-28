use genetic_algorithm::data::{Problema, Solucao};

fn main() {
    let problema = Problema::load_from("instances/ep01.dat");

    let ingredientes:Vec<i32> = vec![0, 1, 2, 3];
    let pressao = 1.0;
    let sol_inicial:Solucao = Solucao::new(ingredientes, &problema, pressao);
    println!("fitness teste {:?}", sol_inicial);

}
