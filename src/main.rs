mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::path::Path;

fn main() {
    let pulse_code = r#"
        source "./pobrane_pliki"

        rule "Faktury i Zakupy Allegro" {
            match extension ["pdf", "txt"]
            match name_contains ["Allegro", "zakup", "faktura"]
            create_folder "./Zakupy_Allegro"
            move to "./Zakupy_Allegro"
        }

        rule "Wszystkie inne Dokumenty i Teksty" {
            match extension ["pdf", "txt", "docx", "doc", "odt"]
            create_folder "./Dokumenty"
            move to "./Dokumenty"
        }

        rule "Wszystkie Grafiki i Zdjęcia" {
            match extension ["png", "jpg", "jpeg", "webp", "gif", "bmp"]
            create_folder "./Media_Grafiki"
            move to "./Media_Grafiki"
        }

        rule "Wszystkie Archiwa i Pakiety" {
            match extension ["zip", "rar", "7z", "deb", "tar", "gz"]
            create_folder "./Archiwa_I_Pakiety"
            move to "./Archiwa_I_Pakiety"
        }

        // Blok domyślny dla absolutnie reszty plików
        fallback {
            create_folder "./Inne_Pliki"
            move to "./Inne_Pliki"
        }
    "#;

    println!("--- URUCHAMIANIE KOMPILATORA I SILNIKA PULSE (SMART SORTER) ---");
    
    let lexer = Lexer::new(pulse_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("[PULSE SOURCE]: Katalog wejściowy: '{}'", program.source_dir);
    println!("[AST INFO]: Załadowano {} inteligentnych reguł.\n", program.rules.len());

    execute_program(&program);

    println!("--- SORTOWANIE ZAKOŃCZONE SUKCESEM ---");
}

fn execute_program(program: &parser::Program) {
    let source_path = Path::new(&program.source_dir);

    if !source_path.exists() {
        let _ = fs::create_dir_all(source_path);
        println!("[INFO]: Utworzono katalog źródłowy '{}'.", program.source_dir);
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
            .unwrap_or("")
            .to_lowercase();

        let mut matched = false;

        // Przechodzimy przez reguły AST
        for rule in &program.rules {
            let mut extension_matched = false;
            let mut name_matched = false;
            let has_extensions = !rule.extensions.is_empty();
            let has_name_filters = !rule.name_filters.is_empty();

            // 1. Sprawdzanie rozszerzenia
            if has_extensions {
                if rule.extensions.iter().any(|e| e.eq_ignore_ascii_case(&file_ext)) {
                    extension_matched = true;
                }
            }

            // 2. Sprawdzanie nazwy
            if has_name_filters {
                if rule.name_filters.iter().any(|filter| {
                    file_name_str.to_lowercase().contains(&filter.to_lowercase())
                }) {
                    name_matched = true;
                }
            }

            // --- INTELIGENTNA LOGIKA DOPASOWANIA ---
            // Reguła pasuje, gdy:
            // - Posiada TYLKO rozszerzenie i ono się zgadza.
            // - Posiada TYLKO filtr nazwy i on się zgadza.
            // - Posiada OBA, i oba się zgadzają (lub wystarczy, że nazwa pasuje precyzyjnie).
            let is_match = match (has_extensions, has_name_filters) {
                (true, false) => extension_matched,
                (false, true) => name_matched,
                (true, true) => (extension_matched && name_matched) || name_matched, // Nazwa ma pierwszeństwo, rozszerzenie wspiera
                (false, false) => false,
            };

            if is_match {
                if let Some(folder) = &rule.create_folder {
                    let _ = fs::create_dir_all(folder);
                }

                if let Some(target) = &rule.move_to {
                    let dest_path = Path::new(target).join(file_name_str);
                    if let Ok(_) = fs::rename(&file_path, &dest_path) {
                        println!("[PRZENIESIONO ({})] '{}' -> {}", rule.name, file_name_str, target);
                        matched = true;
                        break;
                    }
                }
            }
        }

        // Jeśli żaden warunek nie drgnął, uruchamiamy FALLBACK
        if !matched {
            if let Some((fallback_folder, fallback_target)) = &program.fallback {
                let _ = fs::create_dir_all(fallback_folder);
                let dest_path = Path::new(fallback_target).join(file_name_str);
                if let Ok(_) = fs::rename(&file_path, &dest_path) {
                    println!("[PRZENIESIONO (FALLBACK)] '{}' -> {}", file_name_str, fallback_target);
                }
            }
        }
    }
}
