//! Office Angerô: modo leve para computadores com pouca memória.
//!
//! Com até 6 GB de RAM (ou `POWERPOINT_ANGERO_MEMORIA=baixa`), grava `POWERPOINT_ANGERO_MEMORIA=baixa`
//! no ambiente do processo antes de qualquer outra thread existir; o cache de imagens
//! (`deckcraft_render::images`) e o limite do desfazer (`deckcraft_engine`) leem essa variável e
//! guardam menos coisa na memória. `POWERPOINT_ANGERO_MEMORIA=normal` desliga o modo leve.
//! As chamadas ao Windows ficam em `vendor/memoria-angero` (o workspace proíbe `unsafe`).

const VARIAVEL: &str = "POWERPOINT_ANGERO_MEMORIA";
const LIMITE_GB: u64 = 6;

/// Decide o modo de memória; chamar no começo do `main`, antes de criar threads.
pub fn configurar() {
    let escolha = std::env::var(VARIAVEL).unwrap_or_default().to_ascii_lowercase();
    let baixa = match escolha.as_str() {
        "baixa" => true,
        "normal" => false,
        _ => memoria_angero::ram_total().is_some_and(|b| b <= LIMITE_GB << 30),
    };
    if baixa {
        memoria_angero::definir_no_inicio(VARIAVEL, "baixa");
        log::info!("memória: modo leve (pouca RAM): caches de imagem e desfazer menores");
    }
}
