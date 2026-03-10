# 🦀 Mein Rust Lern-Repository

Dieses Repository ist meine persönliche Sammlung, um die Programmiersprache Rust zu lernen. Hier dokumentiere ich Syntax, Konzepte und kleine Test-Skripte.

## 🧠 Was ist Rust?
Rust ist eine moderne, kompilierte Systemprogrammiersprache. Sie kombiniert die extreme Geschwindigkeit von C/C++ mit garantierter Speichersicherheit (Memory Safety). Der Clou: Rust nutzt einen sehr strengen Compiler, der Abstürze und Fehler abfängt, *bevor* das Programm überhaupt startet. Wenn es kompiliert, funktioniert es meistens auch!

## 🛠️ Installation & Setup
Rust wird über das offizielle Tool **`rustup`** installiert.

install : https://rust-lang.org/tools/install/


plan :
```
mein_lern_repo/
├── .gitignore
├── Cargo.toml
├── README.md                <-- Deine Setup-Erklärungen
└── src/
    ├── main.rs              
    └── bin/
        ├── 01_syntax.rs     <-- Dein Skript für Ints, Strings, Variablen
        ├── 02_funktionen.rs <-- Dein Skript für Funktionen
        └── 03_schleifen.rs  <-- Dein Skript für Loops, If/Else
```




## Cargo stuff:
Wie füge ich Pakete (Crates) in Rust hinzu?
In der Rust-Welt nennt man externe Pakete Crates. Das zentrale Verzeichnis dafür (wie PyPI bei Python) ist crates.io. Das Paket für Zufallszahlen heißt in Rust rand.

Du hast zwei Möglichkeiten, es hinzuzufügen:

Der bequeme Weg: <br>
Öffne dein Terminal im Projektordner und tippe:

Bash
```
cargo add rand
```
Was passiert dann? Cargo lädt das Paket herunter und trägt es vollautomatisch mit der aktuellsten Version in deine Cargo.toml unter der Kategorie [dependencies] ein.

Der manuelle Weg:
Du kannst die Cargo.toml auch einfach per Hand im Editor öffnen und es selbst unter [dependencies] reinschreiben:
```
Ini, TOML
[dependencies]
rand = "0.8.5"
```

Wenn du dann das nächste Mal cargo run ausführst, lädt Cargo das Paket automatisch herunter.




## 🏃‍♂️ Wie man Dateien ausführt

In Rust nutzen wir `cargo`, um unseren Code zu kompilieren und zu starten.

* **Das Hauptprojekt starten (`src/main.rs`):**
  ```bash
  cargo run
    ```

* **Ein einzelnes Skript starten (aus dem src/bin/ Ordner):**
    Wenn du eine bestimmte Datei testen willst (z. B. src/bin/00_guess_game.rs), hängst du einfach --bin und den Dateinamen (ohne .rs) an:

    ```
    Bash
    cargo run --bin 00_guess_game
    ```

## 🛠️ Die wichtigsten Cargo-Befehle auf einen Blick

Cargo ist der Paketmanager und das Build-System von Rust. Hier sind die Befehle, die du im Alltag am meisten brauchst:

### 1. Projekte erstellen
* **`cargo new <name>`**: Erstellt ein komplett neues Rust-Projekt in einem neuen Ordner (inklusive `.gitignore`).
* **`cargo init`**: Macht den *aktuellen* Ordner zu einem Rust-Projekt (ideal, wenn man schon ein leeres Git-Repo geklont hat).

### 2. Entwickeln & Ausführen
* **`cargo check`**: Prüft den Code rasend schnell auf Fehler, ohne ein fertiges Programm zu bauen. Nutzt man ständig beim Coden!
* **`cargo run`**: Kompiliert den Code und führt ihn sofort aus. Fühlt sich an wie Python.
* **`cargo run --bin <name>`**: Führt ein einzelnes, separates Skript aus dem `src/bin/`-Ordner aus (ohne `.rs` am Ende).
* **`cargo build`**: Kompiliert das Projekt und legt die ausführbare Datei in `target/debug/` ab.
* **`cargo build --release`**: Kompiliert das Projekt extrem optimiert für maximale Geschwindigkeit. Dauert länger, macht das Programm aber pfeilschnell.

### 3. Pakete (Crates) verwalten
* **`cargo add <paketname>`**: Lädt ein externes Paket (z. B. `rand`) herunter und trägt es automatisch in die `Cargo.toml` ein.
* **`cargo update`**: Schaut im Internet nach kleineren Bugfixes oder Sicherheitsupdates für deine installierten Pakete und aktualisiert die `Cargo.lock`.

### 4. Code aufpolieren (Die Helferlein)
* **`cargo fmt`**: Formatiert deinen gesamten Code automatisch nach dem offiziellen Rust-Standard.
* **`cargo clippy`**: Der Linter von Rust. Gibt dir extrem gute Tipps, wie du deinen Code noch besser, sicherer oder eleganter ("idiomatic") schreiben kannst.

---

## Wie findet man Pakete (Crates) in Rust?

Du musst dich zum Glück nicht mühsam durch Google wühlen! Rust hat ein extrem gut organisiertes und zentrales System dafür – genau wie PyPI (`pip`) bei Python oder `npm` bei JavaScript.

Hier sind die drei wichtigsten Anlaufstellen, die du dir merken solltest:

**1. [crates.io](https://crates.io/) (Die offizielle Zentrale)**
Das ist das absolute Herzstück der Rust-Welt. Wenn du ein Paket suchst (z. B. für Zufallszahlen, Webserver, oder um JSON zu lesen), gibst du das einfach dort in die Suchleiste ein. 
*Tipp:* Achte dort auf die **Anzahl der Downloads** und das **letzte Update**. So siehst du sofort, ob ein Paket der absolute Standard ist oder ein totes Projekt, das seit 4 Jahren nicht mehr gepflegt wird.

**2. [lib.rs](https://lib.rs/) (Der Geheimtipp der Profis)**
Das ist eine inoffizielle Alternative zu crates.io, greift aber auf exakt dieselben Pakete zu. Viele Rust-Entwickler (mich eingeschlossen) finden `lib.rs` viel besser, weil es rasend schnell ist, die Pakete in clevere Kategorien sortiert und dir direkt anzeigt, welche Pakete *wirklich* gut und beliebt sind. Wenn du nicht weißt, welches Paket du für ein Problem nehmen sollst, schau hier rein!

**3. [docs.rs](https://docs.rs/) (Die Dokumentation für alles)**
Sobald du ein Paket gefunden hast, findest du hier **vollautomatisch** die Anleitung dazu. Jeder Code, der auf crates.io hochgeladen wird, bekommt hier automatisch eine generierte Dokumentation.

**Wann nutze ich Google?**
Google (oder Reddit) nutze ich bei Rust eigentlich nur, wenn ich nach Meinungen suche. Also z. B. *"What is the best web framework for Rust 2026?"*. Wenn ich dann die Namen kenne, gehe ich direkt auf `crates.io` oder `lib.rs`, um sie mir anzusehen.



## Wichigiest cargo 