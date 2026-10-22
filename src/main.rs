mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::path::Path;

fn main() {
    // Profesjonalny skrypt Pulse w wersji Pro
    let pulse_code = r#"
        source "./pobrane_pliki"

        // Specjalna reguła dla konkretnych dokumentów zakupowych
        rule "Zakupy i Faktury" {
            match name_contains ["Allegro", "zakup", "faktura", "paragon"]
            move to "./Dokumenty/Zakupy"
        }

        // Reguła Pro: Automatyczne wykrywanie i sortowanie po typach z podfolderami
        rule "Inteligentne Dokumenty" {
            match extension ["pdf", "txt", "docx", "doc", "md", "pptx"]
            move to "./Dokumenty"
        }

        rule "Inteligentne Grafiki" {
            match extension ["png", "jpg", "jpeg", "webp", "svg", "gif"]
            move to "./Grafiki"
        }

        rule "Inteligentne Pakiety i Skrypty" {
            match extension ["deb", "exe", "zip", "rar", "7z", "rsc"]
            move to "./Oprogramowanie_I_Archiwa"
        }

        // Fallback Pro: Dla absolutnie każdego innego pliku automatycznie tworzy folder pod jego rozszerzenie!
        fallback {
            move to "./Inne_Formaty"
        }
    "#;

    println!("--- URUCHAMIANIE SILNIKA PULSE (PRO ARCHITECTURE) ---");
    
    let lexer = Lexer::new(pulse_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("[PULSE SOURCE]: Katalog wejściowy: '{}'", program.source_dir);
    println!("[PRO ENGINE]: Uruchomiono inteligentne mapowanie typów i podfolderów.\n");

    execute_pro_program(&program);

    println!("--- SORTOWANIE PRO ZAKOŃCZONE SUKCESEM ---");
}

fn execute_pro_program(program: &parser::Program) {
    let source_path = Path::new(&program.source_dir);

    if !source_path.exists() {
        let _ = fs::create_dir_all(source_path);
        println!("[INFO]: Utworzono katalog źródłowy '{}'. Wrzuć tam pliki.", program.source_dir);
        return;
    }

    let entries = match fs::read_dir(source_path) {
        Ok(e) => e,
        Err(_) => return,
    };

    let files: Vec<_> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();

    for file_path in files {
        let file_name_str = match file_path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        let file_ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown")
            .to_lowercase();

        let mut matched = false;

        // 1. Sprawdzanie reguł użytkownika z AST
        for rule in &program.rules {
            let mut extension_matched = false;
            let mut name_matched = false;
            let has_extensions = !rule.extensions.is_empty();
            let has_name_filters = !rule.name_filters.is_empty();

            if has_extensions {
                extension_matched = rule.extensions.iter().any(|e| e.eq_ignore_ascii_case(&file_ext));
            }

            if has_name_filters {
                name_matched = rule.name_filters.iter().any(|filter| {
                    file_name_str.to_lowercase().contains(&filter.to_lowercase())
                });
            }

            let is_match = match (has_extensions, has_name_filters) {
                (true, false) => extension_matched,
                (false, true) => name_matched,
                (true, true) => name_matched || extension_matched,
                (false, false) => false,
            };

            if is_match {
                if let Some(base_target) = &rule.move_to {
                    // ARCHITEKTURA PRO: Tworzenie automatycznego podfolderu wg typu pliku (rozszerzenia)
                    // np. ./Grafiki/png, ./Grafiki/svg, ./Dokumenty/pptx
                    let target_dir = Path::new(base_target).join(&file_ext);
                    
                    let _ = fs::create_dir_all(&target_dir);
                    
                    let dest_path = target_dir.join(file_name_str);
                    if let Ok(_) = fs::rename(&file_path, &dest_path) {
                        println!("[PRO MOVE ({})] '{}' -> podfolder /{}", rule.name, file_name_str, file_ext);
                        matched = true;
                        break;
                    }
                }
            }
        }

        // 2. FALLBACK PRO: Jeśli plik nie pasował do żadnej reguły, system sam go przejmuje,
        // tworzy katalog dla jego unikalnego rozszerzenia i segreguje go automatycznie!
        if !matched {
            if let Some((_, fallback_target)) = &program.fallback {
                let target_dir = Path::new(fallback_target).join(&file_ext);
                let _ = fs::create_dir_all(&target_dir);
                
                let dest_path = target_dir.join(file_name_str);
                if let Ok(_) = fs::rename(&file_path, &dest_path) {
                    println!("[PRO FALLBACK AUTO-SORT] '{}' -> Nieznany typ rozdzielony do /{}", file_name_str, file_ext);
                }
            }
        }
    }
}
