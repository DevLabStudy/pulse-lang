# ⚡ Pulse Lang

**Pulse** to autorski, wysokowydajny język skryptowy (DSL – Domain-Specific Language) zaprojektowany z myślą o ultraszybkim web-scrapingu, automatyzacji procesów systemowych oraz dynamicznym monitorowaniu rynku (np. porównywarkach cenowych).

Projekt został stworzony jako zaawansowany dowód kompetencji inżynieryjnych (Proof of Concept) z naciskiem na czystą wydajność, ominięcie zbędnych narzutów tradycyjnych frameworków oraz pełną kontrolę nad kodem.

---

## Kluczowe Cele Projektu
- **Ekstremalna wydajność:** Silnik interpretera pisany w języku systemowym (Rust) z myślą o maksymalnym wykorzystaniu rdzeni procesora.
- **Dedykowana składnia (DSL):** Proste, minimalistyczne komendy stworzone specjalnie do zadań automatyzacji i pobierania danych z sieci.
- **Praktyczne zastosowanie:** Napędzanie silnika autorskiej porównywarki cen z historią wykresów i systemem powiadomień.

---

## Przykładowy Składnia (Pulse)

```pulse
// Przykładowy skrypt monitorujący ceny w systemie Pulse
scrape "[https://sklep.pl/produkt-xyz](https://sklep.pl/produkt-xyz)" {
    selector: ".price",
    store_to: "db_prices"
}

if price < 1500 {
    alert_desktop "Znaleziono obniżkę cenową!"
}
