mod lexer;
use lexer::{Lexer, Token};
use std::fs;
use std::path::Path;

fn main() {
    // Rozbudowany skrypt Pulse z wieloma formatami i kategoriami
    let pulse_code = r#"
        source "./pobrane_pliki"

        rule "Dokumenty i Faktury" {
            match extension ["pdf", "txt", "docx"]
            create_folder "./Kategoria_Dokumenty"
            move to "./Kategoria_Dokumenty"
        }

        rule "Media i Grafiki" {
            match extension ["png", "jpg", "jpeg", "webp"]
            create_folder "./Kategoria_Media"
            move to "./Kategoria_Media"
        }

        rule "Archiwa i Pliki ZIP" {
            match extension ["zip", "rar", "7z"]
            create_folder "./Kategoria_Archiwa"
            move to "./Kategoria_Archiwa"
        }
    "#;

    println!("--- URUCHAMIANIE SILNIKA PULSE (ADVANCED SORTER) ---");
    run_pulse_sorter(pulse_code);
}

fn run_pulse_sorter(input: &str) {
    let mut lexer = Lexer::new(input);
    
    let mut source_dir = String::new();
    let mut target_folder = String::new();

    loop {
        let token = lexer.next_token();
        match token {
            Token::Source => {
                if let Token::String(path) = lexer.next_token() {
                    source_dir = path;
                    println!("[PULSE SOURCE]: Katalog wejściowy: '{}'", source_dir);
                }
            }
            Token::Extension => {
                // Sprawdzamy, czy mamy pojedyncze rozszerzenie czy tablicę rozszerzeń
                let next = lexer.next_token();
                let mut extensions = Vec::new();

                match next {
                    Token::String(ext) => {
                        extensions.push(ext);
                    }
                    Token::LBrace | Token::Identifier(_) => {
                        // Obsługa prostego wczytywania listy rozszerzeń
                        loop {
                            let item = lexer.next_token();
                            match item {
                                Token::String(e) => extensions.push(e),
                                Token::RBrace => break,
                                Token::Eof => break,
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }

                // Czekamy na akcję utworzenia folderu i przeniesienia
                // W uproszczonym interpreterze łapiemy kolejne tokeny reguły
                let mut local_target = String::new();
                let mut found_move = false;

                loop {
                    let t = lexer.next_token();
                    match t {
                        Token::CreateFolder => {
                            if let Token::String(folder) = lexer.next_token() {
                                let _ = fs::create_dir_all(&folder);
                                local_target = folder.clone();
                                target_folder = folder;
                                println!("[PULSE FOLDER]: Utworzono: '{}'", target_folder);
                            }
                        }
                        Token::Move => {
                            if lexer.next_token() == Token::To {
                                if let Token::String(dest) = lexer.next_token() {
                                    local_target = dest.clone();
                                    target_folder = dest;
                                    found_move = true;
                                }
                            }
                        }
                        Token::Rule | Token::Eof => {
                            // Cofamy lub kończymy regułę, wykonujemy sortowanie dla tych rozszerzeń
                            if found_move && !extensions.is_empty() {
                                for ext in &extensions {
                                    execute_file_sorting(&source_dir, ext, &local_target);
                                }
                            }
                            // Jeśli to początek kolejnej reguły, musimy obsłużyć token
                            if t == Token::Rule {
                                // Małe mini-cofnięcie logiki dla kolejnej reguły
                                break;
                            }
                            if t == Token::Eof {
                                return;
                            }
                        }
                        _ => {}
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
    
    if !path.exists() {
        let _ = fs::create_dir_all(path);
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
                                println!("[PRZENIESIONO]: {:?} (.({})) -> {}", file_name, extension, target);
                            }
                        }
                    }
                }
            }
        }
    }
}
