//! Office Angerô: tradução da interface num ponto só.
//!
//! O programa registra uma função (`definir_tradutor`) que recebe o texto em inglês e devolve a
//! tradução, se houver. Todo texto que vira `RichText`/`WidgetText` ou é desenhado por
//! `Painter::text`/`Painter::layout*` passa por aqui. O que o usuário digita (`TextEdit`) não passa.

use std::sync::OnceLock;

/// Devolve a tradução de um texto exato da interface, ou `None` para manter o original.
pub type Tradutor = fn(&str) -> Option<&'static str>;

static TRADUTOR: OnceLock<Tradutor> = OnceLock::new();

/// Registra o tradutor (só a primeira chamada vale).
pub fn definir_tradutor(f: Tradutor) {
    let _ = TRADUTOR.set(f);
}

/// O texto traduzido, ou o próprio texto.
#[inline]
pub fn traduzir(texto: String) -> String {
    match TRADUTOR.get() {
        Some(f) if !texto.is_empty() => f(&texto).map_or(texto, str::to_owned),
        _ => texto,
    }
}
