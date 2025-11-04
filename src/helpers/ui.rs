// ui.rs
use color_eyre::eyre::{Context, Result};
use once_cell::sync::Lazy;
use rustyline::{DefaultEditor, error::ReadlineError, history::History};
use std::sync::Mutex;

/// Editor global para conservar historial en todo el programa.
static RL: Lazy<Mutex<DefaultEditor>> = Lazy::new(|| {
    let ed = DefaultEditor::new().expect("No se pudo crear Editor");
    Mutex::new(ed)
});
/// Lee una línea con edición (flechas), historial y prompt.
/// Ok(Some(s))  -> línea ingresada
/// Ok(None)    -> usuario terminó (EOF / "exit" opcional)
/// Err(e)      -> error real
pub fn prompt(prompt: &str) -> Result<Option<String>> {
    let mut rl = RL.lock().expect("Poisoned mutex");
    match rl.readline(prompt) {
        Ok(line) => {
            let input = line.trim().to_string();
            if input.is_empty() {
                return Ok(Some(input)); // vacío permitido; decide arriba qué hacer
            }
            // Guarda en historial (para ↑/↓)
            rl.add_history_entry(input.as_str()).ok();
            Ok(Some(input))
        }
        Err(ReadlineError::Eof) => Ok(None), // Ctrl+D
        Err(ReadlineError::Interrupted) => Ok(Some(String::new())), // Ctrl+C -> línea vacía
        Err(e) => Err(e).context("[ui::prompt] - Error de entrada"),
    }
}

// Variante que obliga a texto no vacío. Devuelve None en EOF.
pub fn prompt_nonempty(text: &str) -> Result<Option<String>> {
    loop {
        match prompt(text)? {
            Some(s) if !s.trim().is_empty() => return Ok(Some(s)),
            Some(_) => { /* vacío: vuelve a pedir */ }
            None => return Ok(None), // EOF
        }
    }
}
