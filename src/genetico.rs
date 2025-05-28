use crate::data::{solucao::populacao_inicial, Problema, Solucao};

fn get_melhor_solucao(populacao: &Vec<Solucao>) -> Solucao{
    let mut melhor_solucao: &Solucao;
    let mut tmp = populacao.iter();
    melhor_solucao = tmp.next().unwrap();
    while let Some(i) = tmp.next() {
        if i.resultado > melhor_solucao.resultado{
            melhor_solucao = i;
        }
    }
    melhor_solucao.clone()
}

pub fn genetico(
    problema: &Problema, 
    pressao: f64, 
    tamanho_populacao: usize) -> Solucao{
    // Cria a população inicial e calcula seu fitness
    let melhor_solucao: Solucao;
    let populacao: Vec<Solucao> = populacao_inicial(tamanho_populacao, pressao, &problema);

    // while condição de parada
        // Seleção
            // seleciona os pais que irão cruzar

            //cruzamento

            // seleciona quem irá para a proxima geração
            // nova população

        // mutação na nova população

    melhor_solucao = get_melhor_solucao(&populacao);
    melhor_solucao
}
