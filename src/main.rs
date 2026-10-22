mod lexer;
use lexer::{Lexer, Token};
use std::fs;
use std::path::Path;

fn main() {
    // Skrypt w Twoim języku Pulse do sortowania plików
    let pulse_code = r#"
        source "./pobrane_pliki"

        rule "Dokumenty Zakupowe PDF" {
            match extension "pdf"
            create_folder "./Zakupy_PDF"
            move to "./Zakupy_PDF"
        }

        rule "Grafiki i Obrazy" {
            match extension "png"
            create_folder "./Obrazy"
            move to "./Obrazy"
        }
    "#;

    println!("--- URUCHAMIANIE SILNIKA PULSE (FILE SORTER) ---");
    run_pulse_sorter(pulse_code);
}

fn run_pulse_sorter(input: &str) {
    let mut lexer = Lexer::new(input);
    
    let mut source_dir = String::new();
    let mut current_ext = String::new();
    let mut target_folder = String::new();

    loop {
        let token = lexer.next_token();
        match token {
            Token::Source => {
                if let Token::String(path) = lexer.next_token() {
                    source_dir = path;
                    println!("[PULSE SOURCE]: Ustawiono katalog wejściowy: '{}'", source_dir);
                }
            }
            Token::Extension => {
                if let Token::String(ext) = lexer.next_token() {
                    current_ext = ext;
                }
            }
            Token::CreateFolder => {
                if let Token::String(folder) = lexer.next_token() {
                    if let Err(e) = fs::create_dir_all(&folder) {
                        eprintln!("[BŁĄD]: Nie udało się utworzyć folderu {}: {}", folder, e);
                    } else {
                        println!("[PULSE FOLDER]: Utworzono katalog: '{}'", folder);
                    }
                }
            }
            Token::Move => {
                let next = lexer.next_token();
                if next == Token::To {
                    if let Token::String(dest) = lexer.next_token() {
                        target_folder = dest;
                        
                        // Wykonaj fizyczne sortowanie plików
                        execute_file_sorting(&source_dir, &current_ext, &target_folder);
                    }
                }
            }
            Token::Eof => break,
            _ => {}
        }
    }
    println!("--- SORTOWANIE ZAKOŃCZONE SUKCESEM ---");
}

fn execute_file_sorting(source: &str, extension: &str, target: &str) {
    let path = Path::new(source);
    
    // Jeśli folder źródłowy nie istnieje, stwórz go automatycznie
    if !path.exists() {
        let _ = fs::create_dir_all(path);
        println!("[INFO]: Utworzono folder '{}'. Wrzuć do niego pliki i uruchom ponownie cargo run.", source);
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                    if ext.eq_ignore_ascii_case(extension) {
                        if let Some(file_name) = file_path.file_name() {
                            let dest_path = Path::new(target).join(file_name);
                            if let Ok(_) = fs::rename(&file_path, &dest_path) {
                                println!("[PRZENIESIONO]: {:?} -> {}", file_name, target);
                            }
                        }
                    }
                }
            }
        }
    }
}
