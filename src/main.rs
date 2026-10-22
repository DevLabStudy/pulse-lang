mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::path::Path;

fn main() {
    // Zaawansowany skrypt w języku Pulse z wieloma regułami, formatami i filtrami nazw
    let pulse_code = r#"
        source "./pobrane_pliki"

        rule "Faktury i Zakupy Allegro" {
            match extension ["pdf", "txt"]
            match name_contains ["Allegro", "zakup", "faktura"]
            create_folder "./Zakupy_Allegro"
            move to "./Zakupy_Allegro"
        }

        rule "Inne Dokumenty PDF" {
            match extension "pdf"
            create_folder "./Pozostale_PDF"
            move to "./Pozostale_PDF"
        }

        rule "Grafiki i Zdjęcia" {
            match extension ["png", "jpg", "jpeg", "webp"]
            create_folder "./Media_Grafiki"
            move to "./Media_Grafiki"
        }
    "#;

    println!("--- URUCHAMIANIE KOMPILATORA I SILNIKA PULSE (AST) ---");
    
    // 1. Lexer przetwarza tekst na tokeny
    let lexer = Lexer::new(pulse_code);
    
    // 2. Parser buduje drzewo AST (Program i Reguły)
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("[PULSE SOURCE]: Katalog wejściowy: '{}'", program.source_dir);
    println!("[AST INFO]: Znaleziono {} reguł w kodzie źródłowym.\n", program.rules.len());

    // 3. Silnik wykonawczy przetwarza pliki zgodnie z drzewem AST
    execute_program(&program);

    println!("--- SORTOWANIE ZAKOŃCZONE SUKCESEM ---");
}

fn execute_program(program: &parser::Program) {
    let source_path = Path::new(&program.source_dir);

    if !source_path.exists() {
        let _ = fs::create_dir_all(source_path);
        println!("[INFO]: Utworzono katalog źródłowy '{}'. Wrzuć tam pliki i uruchom ponownie cargo run.", program.source_dir);
        return;
    }

    let entries = match fs::read_dir(source_path) {
        Ok(e) => e,
        Err(_) => return,
    };

    // Zbieramy listę plików w folderze
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

        // Sprawdzamy każdą regułę z drzewa AST po kolei
        for rule in &program.rules {
            let mut matches_ext = true;
            let mut matches_name = true;

            // Sprawdzanie rozszerzenia (jeśli zdefiniowano w regule)
            if !rule.extensions.is_empty() {
                matches_ext = rule.extensions.iter().any(|e| e.eq_ignore_ascii_case(&file_ext));
            }

            // Sprawdzanie słów kluczowych w nazwie (jeśli zdefiniowano w regule)
            if !rule.name_filters.is_empty() {
                matches_name = rule.name_filters.iter().any(|filter| {
                    file_name_str.to_lowercase().contains(&filter.to_lowercase())
                });
            }

            // Jeśli plik pasuje do warunków reguły
            if matches_ext && matches_name {
                if let Some(folder) = &rule.create_folder {
                    let _ = fs::create_dir_all(folder);
                }

                if let Some(target) = &rule.move_to {
                    let dest_path = Path::new(target).join(file_name_str);
                    if let Ok(_) = fs::rename(&file_path, &dest_path) {
                        println!("[PRZENIESIONO ({})] '{}' -> {}", rule.name, file_name_str, target);
                        // Przeniesiono – przerywamy sprawdzanie kolejnych reguł dla tego pliku
                        break;
                    }
                }
            }
        }
    }
}
