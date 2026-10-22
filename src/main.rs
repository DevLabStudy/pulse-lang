mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::{Parser, Command, Expr};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

fn main() {
    println!("=== INTELIGENTNY SILNIK PULSE (WEB SCRAPER) ===");

    let url = prompt_user("Podaj adres URL (np. https://pl.wikipedia.org/wiki/Tkanka): ");
    let keyword = prompt_user("Podaj słowo kluczowe do wyszukania (np. nabłonek, krew): ");
    let filename = prompt_user("Podaj nazwę pliku wynikowego (np. biologia.txt): ");

    let source_code = format!(
        r#"
            log "Startuje silnik Pulse..."
            let surowe_dane = scrape "{}" {{
                selector: "body"
            }}
            log "Pobieranie zakończone."
        "#,
        url
    );

    let mut lexer = Lexer::new(&source_code);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let commands = parser.parse();

    let mut variables: HashMap<String, Vec<String>> = HashMap::new();

    println!("\n--- WYKONANIE PROGRAMU PULSE ---");
    for cmd in commands {
        execute_command(cmd, &mut variables);
    }

    // Zaawansowane przeszukiwanie całego tekstu strony
    if let Some(items) = variables.get("surowe_dane") {
        let keyword_lower = keyword.to_lowercase();
        let mut matched_sentences = Vec::new();

        for block in items {
            // Dzielimy pobrany tekst na mniejsze fragmenty (zdania/linie)
            for sentence in block.split(['.', '!', '?']) {
                if sentence.to_lowercase().contains(&keyword_lower) {
                    let cleaned = sentence.trim().to_string();
                    if !cleaned.is_empty() && !matched_sentences.contains(&cleaned) {
                        matched_sentences.push(cleaned);
                    }
                }
            }
        }

        let count = matched_sentences.len();
        let content = if count > 0 {
            matched_sentences.join(".\n\n") + "."
        } else {
            format!("Nie znaleziono dokładnego dopasowania dla słowa: '{}'", keyword)
        };

        match fs::write(&filename, content) {
            Ok(_) => println!("[SAVE]: Zapisano wyniki do pliku '{}' (znaleziono pasujących fragmentów: {})", filename, count),
            Err(e) => eprintln!("[BŁĄD]: Nie udało się zapisać pliku: {}", e),
        }
    } else {
        eprintln!("[BŁĄD]: Nie udało się pobrać danych ze strony.");
    }
}

fn prompt_user(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn execute_command(cmd: Command, variables: &mut HashMap<String, Vec<String>>) {
    match cmd {
        Command::Log(expr) => match expr {
            Expr::String(msg) => {
                println!("[LOG]: {}", msg);
            }
            Expr::Variable(var_name) => {
                if let Some(val) = variables.get(&var_name) {
                    println!("[LOG (zmienna '{}')]: {:?}", var_name, val);
                } else {
                    eprintln!("[BŁĄD]: Nieznana zmienna '{}'", var_name);
                }
            }
        },
        Command::Scrape { url, selector } => {
            let _ = run_scrape(&url, &selector);
        }
        Command::Let { name, command } => {
            if let Command::Scrape { url, selector } = *command {
                println!("[SCRAPE]: Łączę się z adresem '{}'...", url);
                let scraped_data = run_scrape(&url, &selector);
                variables.insert(name, scraped_data);
            } else {
                eprintln!("[BŁĄD]: Konstrukcja 'let' obsługuje obecnie tylko wyniki z polecenia 'scrape'");
            }
        }
        Command::For { variable, collection, body } => {
            if let Some(items) = variables.get(&collection).cloned() {
                for item in items {
                    variables.insert(variable.clone(), vec![item]);
                    for cmd in &body {
                        execute_command(cmd.clone(), variables);
                    }
                }
            } else {
                eprintln!("[BŁĄD]: Nieznana kolekcja w pętli for: '{}'", collection);
            }
        }
        Command::Save { variable, filename } => {
            if let Some(items) = variables.get(&variable) {
                let content = items.join("\n");
                let _ = fs::write(&filename, content);
            }
        }
    }
}

// Funkcja pobierająca stronę z nagłówkiem User-Agent (wymaganym przez Wikipedię)
fn run_scrape(url: &str, selector_str: &str) -> Vec<String> {
    let mut results = Vec::new();
    
    // Tworzymy klienta HTTP z wymaganym User-Agentem, aby Wikipedia nas nie odrzuciła
    let client = match reqwest::blocking::Client::builder()
        .user_agent("PulseScraper/1.0 (Educational Project)")
        .build()
    {
        Ok(c) => c,
        Err(_) => return results,
    };

    match client.get(url).send() {
        Ok(resp) => {
            if let Ok(body) = resp.text() {
                let document = Html::parse_document(&body);
                if let Ok(sel) = Selector::parse(selector_str) {
                    for element in document.select(&sel) {
                        let text = element.text().collect::<Vec<_>>().join(" ");
                        let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
                        if !cleaned.is_empty() {
                            results.push(cleaned);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[BŁĄD]: Nie udało się pobrać strony: {}", e);
        }
    }
    results
}
