//! Office Angerô: as duas chamadas que precisam de `unsafe`, isoladas aqui porque o workspace
//! proíbe `unsafe` (`forbid(unsafe_code)`): a RAM total do Windows e gravar uma variável de
//! ambiente no começo do `main` (no Rust 2024, `set_var` é `unsafe`).

/// RAM física total, em bytes (só Windows).
#[cfg(target_os = "windows")]
pub fn ram_total() -> Option<u64> {
    #[repr(C)]
    struct MemoryStatusEx {
        length: u32,
        memory_load: u32,
        total_phys: u64,
        avail_phys: u64,
        total_page_file: u64,
        avail_page_file: u64,
        total_virtual: u64,
        avail_virtual: u64,
        avail_extended_virtual: u64,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GlobalMemoryStatusEx(buffer: *mut MemoryStatusEx) -> i32;
    }
    let mut m = MemoryStatusEx {
        length: std::mem::size_of::<MemoryStatusEx>() as u32,
        memory_load: 0,
        total_phys: 0,
        avail_phys: 0,
        total_page_file: 0,
        avail_page_file: 0,
        total_virtual: 0,
        avail_virtual: 0,
        avail_extended_virtual: 0,
    };
    // SAFETY: `m` é um MEMORYSTATUSEX válido com `length` preenchido, como a API pede.
    (unsafe { GlobalMemoryStatusEx(&mut m) } != 0).then_some(m.total_phys)
}

#[cfg(not(target_os = "windows"))]
pub fn ram_total() -> Option<u64> {
    None
}

/// Grava uma variável de ambiente do processo. Chamar só no começo do `main`, antes de criar
/// threads (é o que torna o `set_var` seguro).
pub fn definir_no_inicio(chave: &str, valor: &str) {
    // SAFETY: quem chama garante que ainda só existe a thread principal.
    unsafe { std::env::set_var(chave, valor) };
}
