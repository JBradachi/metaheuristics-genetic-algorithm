use rand::Rng;

use crate::data::{solucao::populacao_inicial, Problema, Solucao};

const AR: f64 = 0.2;

fn get_melhor_solucao(populacao: &Vec<Solucao>) -> Solucao {
    let mut melhor_solucao: &Solucao;
    let mut tmp = populacao.iter();
    melhor_solucao = tmp.next().unwrap();
    while let Some(i) = tmp.next() {
        if i.resultado > melhor_solucao.resultado {
            melhor_solucao = i;
        }
    }
    melhor_solucao.clone()
}

fn get_melhor_solucao_i(populacao: &Vec<Solucao>) -> (Solucao, usize) {
    let mut melhor_solucao: &Solucao;
    let mut tmp = populacao.iter();
    melhor_solucao = tmp.next().unwrap();
    let mut indice: usize = 0;
    while let Some(i) = tmp.next() {
        if i.resultado > melhor_solucao.resultado {
            melhor_solucao = i;
        }
        indice += 1;
    }
    (melhor_solucao.clone(), indice)
}

fn crossover(problema: &Problema, pais: &Vec<Solucao>) -> Vec<Solucao> {
    let mut filhos = Vec::new();
    while filhos.len() < pais.len() {
        let mut rng = rand::thread_rng();
        let p0 = rng.choose(&pais).unwrap();
        let p1 = rng.choose(&pais).unwrap();
        let corte: usize = rng.gen_range(1, problema.num_ingred - 2);

        let n = if p0.ingredientes.len() > p1.ingredientes.len() {
            p0.ingredientes.len()
        } else {
            p1.ingredientes.len()
        };
        let mut f0 = Vec::new();
        let mut f1 = Vec::new();
        for i in 0..n {
            if i < p0.ingredientes.len() {
                let ing0 = p0.ingredientes[i];
                if ing0 < corte {
                    f0.push(ing0);
                } else {
                    f1.push(ing0);
                }
            }
            if i < p1.ingredientes.len() {
                let ing1 = p1.ingredientes[i];
                if ing1 < corte {
                    f1.push(ing1);
                } else {
                    f0.push(ing1);
                }
            }
        }
        let s0 = Solucao::new(f0, problema);
        let s1 = Solucao::new(f1, problema);
        filhos.push(s0);
        filhos.push(s1);
    }
    filhos
}

fn bit_flip(problema: &Problema, mut populacao: Vec<Solucao>) -> Vec<Solucao> {
    let mut rng = rand::thread_rng();
    let mut tmp = populacao.iter_mut();
    while let Some(i) = tmp.next() {
        let r = rng.gen_range(0.0, 1.0);

        //verifica se receberá a mutação
        if r >= AR {
            //faz a mutação
            let gene_mutado: usize = rng.gen_range(1, problema.num_ingred - 2);
            if i.ingredientes.contains(&gene_mutado) {
                i.ingredientes.retain(|&x| x != gene_mutado);
            } else {
                i.ingredientes.push(gene_mutado);
            }
            i.update(problema);
        }
    }
    populacao
}

fn mutacao(
    problema: &Problema,
    filhos: Vec<Solucao>,
) -> Vec<Solucao> {

    let filhos_mutados: Vec<Solucao> = bit_flip(problema, filhos.clone());

    filhos_mutados
}

fn realiza_torneio(competidores: Vec<Solucao>) -> Solucao {
    let mut campeao_atual: Solucao = competidores[0].clone();
    let tamanho_torneio = competidores.len();
    for i in 1..tamanho_torneio {
        if competidores[i].resultado > campeao_atual.resultado {
            campeao_atual = competidores[i].clone();
        }
    }

    campeao_atual
}

fn selecao(populacao: &Vec<Solucao>) -> Vec<Solucao> {
    let tamanho_torneio = 3;
    let mut rng = rand::thread_rng();

    let mut populacao_final: Vec<Solucao> = Vec::new();
    let mut competidores: Vec<Solucao> = Vec::new();

    for _ in 0..populacao.len() {
        for _ in 0..tamanho_torneio {
            let competidor: &Solucao = rng.choose(&populacao).unwrap();
            competidores.push(competidor.clone());
        }
        populacao_final.push(realiza_torneio(competidores.clone()));
        competidores.clear();
    }

    populacao_final
}

fn elitismo(problema: &Problema, pais: &mut Vec<Solucao>, filhos: &mut Vec<Solucao>) {
    let k_pais = 3;
    let mut rng = rand::thread_rng();

    for _ in 0..k_pais {
        let (mut melhor_pai, i) = get_melhor_solucao_i(pais);
        // melhor_pai.update(problema);
        pais.remove(i);

        let filho_removido: usize = rng.gen_range(0, filhos.len());
        filhos.remove(filho_removido);
        filhos.push(melhor_pai);
    }
}

pub fn genetico(problema: &mut Problema, tamanho_populacao: usize) -> (Solucao, Solucao) {
    // Cria a população inicial e calcula seu fitness
    let mut populacao: Vec<Solucao> = populacao_inicial(tamanho_populacao, problema);
    let mut melhor_solucao: Solucao = get_melhor_solucao(&populacao);
    let solucao_inicial: Solucao = melhor_solucao.clone();
    let mut resultado_anterior = melhor_solucao.resultado;

    let mut iteracao_sem_mudanca: i32 = 0;
    while iteracao_sem_mudanca <= 100 {
        // (n° x de iterações sem mudar o melhor indivíduo)

        let mut pais: Vec<Solucao> = selecao(&populacao);

        let filhos: Vec<Solucao> = crossover(problema, &pais);

        populacao = mutacao(problema, filhos);

        elitismo(&problema, &mut pais, &mut populacao);

        melhor_solucao = get_melhor_solucao(&populacao);

        if melhor_solucao.resultado == resultado_anterior {
            iteracao_sem_mudanca += 1;
        } else {
            resultado_anterior = melhor_solucao.resultado;
            iteracao_sem_mudanca = 0;
        }
        println!("Acabou uma iteração ({iteracao_sem_mudanca})");
        // problema.pressao *= 1.05
    }

    (solucao_inicial, melhor_solucao)
}
