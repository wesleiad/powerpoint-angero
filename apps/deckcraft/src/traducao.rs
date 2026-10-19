//! Office Angerô: interface do PowerPoint em português do Brasil.
//!
//! A tabela `traducao_pt.tsv` (inglês<TAB>português) é embutida no programa e registrada no egui
//! (`egui::traducao`), que traduz todo texto de interface na hora de desenhar. Com
//! `POWERPOINT_ANGERO_IDIOMA=en` a interface fica em inglês.

use std::collections::HashMap;
use std::sync::OnceLock;

static TABELA: OnceLock<HashMap<String, &'static str>> = OnceLock::new();

fn desescapar(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\t", "\t")
}

fn tabela() -> &'static HashMap<String, &'static str> {
    TABELA.get_or_init(|| {
        include_str!("traducao_pt.tsv")
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| l.split_once('\t'))
            .map(|(en, pt)| (desescapar(en), &*Box::leak(desescapar(pt).into_boxed_str())))
            .collect()
    })
}

fn traduzir(texto: &str) -> Option<&'static str> {
    tabela().get(texto).copied()
}

/// Liga a tradução (antes de abrir a janela).
pub fn ativar() {
    if std::env::var("POWERPOINT_ANGERO_IDIOMA").is_ok_and(|v| v.eq_ignore_ascii_case("en")) {
        return;
    }
    egui::traducao::definir_tradutor(traduzir);
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_tabela_traduz_textos_da_faixa_de_opcoes() {
        assert_eq!(super::traduzir("Insert"), Some("Inserir"));
        assert_eq!(super::traduzir("New\nSlide"), Some("Novo\nSlide"));
        assert_eq!(super::traduzir("texto que não existe"), None);
    }
}
