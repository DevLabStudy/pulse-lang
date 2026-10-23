# ⚡ Pulse Lang

**Pulse** to autorski, wysokowydajny język skryptowy (DSL – Domain-Specific Language) zaprojektowany z myślą o zaawansowanej automatyzacji procesów systemowych, inteligentnym sortowaniu plików oraz zarządzaniu strukturą katalogów.

Projekt został stworzony jako zaawansowany dowód kompetencji inżynieryjnych (Proof of Concept) z naciskiem na czystą wydajność, bezpieczeństwo operacji na plikach oraz pełną kontrolę nad kodem.

---

## 🛠️ Zgłaszanie Błędów
Wszelkie zauważone błędy, anomalie w działaniu silnika lub problemy z przetwarzaniem danych można zgłaszać bezpośrednio na adresy e-mail:
- **devlabstudy@devlabstudy.ovh**
- **bledy@devlabstudy.ovh**

---

## Kluczowe Cele Projektu
- **Ekstremalna wydajność:** Silnik interpretera pisany w języku systemowym (Rust) z myślą o maksymalnym wykorzystaniu rdzeni procesora.
- **Dedykowana składnia (DSL):** Proste, minimalistyczne komendy stworzone specjalnie do zadań automatyzacji i reguł plikowych.
- **Bezpieczeństwo danych:** Wbudowane mechanizmy rekurencyjnego skanowania podfolderów oraz ochrona przed cichym nadpisywaniem plików (automatyczne doklejanie unikalnych sufiksów numerycznych przy konflikcie nazw).

---

## Przykładowa Składnia (Pulse)

```pulse
// Przykładowy skrypt sortujący pliki w systemie Pulse
source "./pobrane_pliki"

rule "Zakupy i Faktury" {
    match name_contains ["Allegro", "zakup", "faktura", "paragon"]
    move to "./Dokumenty/Zakupy"
}

rule "Inteligentne Dokumenty" {
    match extension ["pdf", "txt", "docx", "doc", "md", "pptx"]
    move to "./Dokumenty"
}

fallback {
    move to "./Inne_Formaty"
}
