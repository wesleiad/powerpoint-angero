# PowerPoint - Angerô

Programa do **Office Angerô**, o pacote de escritório do **Grupo Angerô**.

Apresentações: slides com temas, imagens, gráficos e animações; abre e salva .pptx.

- Ajuda: https://github.com/wesleiad/Office-Angero/blob/main/docs/ajuda/powerpoint.md
- Office Angerô (ícones, instaladores, ajuda): https://github.com/wesleiad/Office-Angero
- Versões compiladas: aba **Releases** deste repositório (o programa instalado se atualiza sozinho a partir delas).

## Compilar

Windows, com Rust (MSVC) e Visual Studio Build Tools (C++):

```
cargo build --release --locked -p deckcraft -p deckcraft-cli
```

Gera `target/release/powerpoint-angero.exe` e `target/release/powerpoint-angero-cli.exe`.
A cada push no ramo `angero`, o GitHub Actions compila e publica uma versão nova em Releases.

## Código-base e licença

Desenvolvido por Weslei Domingues (TI) para o Grupo Angerô, a partir do código aberto do
**DeckCraft** (projeto ArtCraft, https://github.com/storytold/deckcraft), usado como base.
Licença: MIT OU Apache-2.0, à escolha — veja `LICENSE-MIT`, `LICENSE-APACHE` e `NOTICE`.
