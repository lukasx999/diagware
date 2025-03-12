# Multithreading

Während dem Durchführen von Diagnosen ist es wichtig, dass die Diagnose nebenläufig zum UI durchgeführt wird.
Würde die Diagnose synchron ausgeführt werden, so würde das Rendering und Input-Handling des UI's blockieren, bis die Diagnose vollendet ist.
Daher wird die Diagnose parallel zum UI in einem seperaten Thread ausgeführt.

# Rust

Die Software wurde in der Programmiersprache Rust implementiert.

Eigenschaften der Sprache:
- Strong typing
- Static typing
- Kompiliert (mit rustc)
- Memory / Threadsicher durch den Borrowchecker

Installieren von einer Rust-Toolchain:
[Rustup](rustup.rs)

Durch Rustup wird Rustup, Rustc und Cargo automatisch installiert.

## Cargo

Cargo ist ein Dependency- und Projectmanager für Rust.
Cargo fungiert ebenso als Buildsystem.

Für Rust Projekte ist es empfohlen den Compiler nicht direkt zu verwenden, sondern diese Aufgabe an Cargo abzugeben.

Die wichtigsten Cargo Befehle:

```sh
$ cargo init            # Projekt im jetzigen Verzeichnis initialisieren
$ cargo new             # Projekt in einem neuen Verzeichnis initialisieren
$ cargo build           # Projekt im Debug-Modus bauen
$ cargo build --release # Projekt im Release-Modus bauen
$ cargo run             # Projekt im Debug-Modus bauen und ausführen
$ cargo check           # Projekt nur überprüfen, ohne zu bauen
```


# GUI

## GUI Paradigmen: Immediate-mode GUI vs Retained-mode GUI

### Immediate-mode GUI
- z.B.: DearImgui, Egui

Das UI wird zu jedem Frame neu gezeichnet. Dabei wird zu jedem Frame eine benutzerdefinierte Funktion aufgerufen, welche die UI-Komponenten rendert.

**Vorteile**:
- Einfachere Programmierung

**Nachteile**:
- Keine komplizierten Layouts möglich

### Retained-mode GUI
- z.B.: GTK, Qt

UI-Komponenten werden einmalig festgelegt, und Callback-Funktionen werden für diese Komponenten festgelegt.

**Vorteile**:
- Komplizierte Layouts möglich

**Nachteile**:
- Komplizierte Programmiernung
  - Arbeiten mit State in Callbacks ist oft kompliziert
- Oft ein UI-Designer notwendig
- Ist oft Teil von großen Frameworks, die auch andere Teile der Entwicklung vorgeben (zB Qt: QMake, QtDesigner, QSql, MOC (Meta Object Compiler (C++ Syntax Erweiterungen) teil des Build-Vorgangs)


## Egui

Für die Implementation der Benutzeroberfläche wurde das Egui Crate für Rust gewählt.
Egui rendert jediglich Dreiecke, und ist sich selbst daher nicht bewusst auf welcher Oberfläche es selbst aktiv ist.
Für die tatsächliche Entwicklung mit egui wird eframe benötigt, ein weiteres Crate, welches ein Fenster öffnet, und einen egui Kontext zur Verfügung stellt.

Eframe unterstützt folgende Platformen:
- Web
- Linux
- Windows
- Mac
- Android


### Beispielanwendung mit Egui:

Ein Struct wird definiert, welches den Zustand der Applikation beinhaltet.

```rust
struct GuiApplication;

impl GuiApplication {
    pub fn new() -> Self { Self }
}
```

Anschließend wird der App Trait von eframe implementiert, welcher die update Funktion beinhaltet.
Die update Funktion wird für jeden Frame der Applikation aufgerufen.

```rust
impl eframe::App for GuiApplication {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.button("Click me!");
        });

    }

}
```

Um die Applikation auszuführen, muss nur noch Glue-Code geschrieben werden, um die Applikation zu initialisieren.

```rust
fn main() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "Example",
        options,
        Box::new(|cc| {
            Ok(Box::new(GuiApplication::new()))
        }),
    )

}
```




# Datenbank

Für das Speichern der Sollwerte und Modulkonfigurationen wird eine Datenbank benötigt.
In diesem Falle wird Sqlite3 verwendet.

### Vorteile:
- Die gesamte DB ist in einer einzelnen Datei enthalten (database.db)
  - DB ist portabel
  - Arbeiten mit DB ist einfacher
- Es sind keine Server-Prozesse wie bei zB Postgres oder MySQL notwendig

### Nachteile:
- Ungeeignet für große Anwendungen (Nicht der Fall)
- Kein Zugriff auf DB durch mehrere Benutzer (Kein Problem)

## Implementation

Implementiert wurde der Datenbank-Zugriff mit dem SQLx Crate in Rust.
SQLx stellt Makros zur Verfügung, welche die Abfragen auf die Datenbank zur Compiletime überprüfen können.
Dies ist ein großer Vorteil und erleichtert die Entwicklung, da somit Fehler bei den Abfragen früher im Entwicklungsprozess (beim Kompilieren) gefunden werden können, und nicht zur Runtime.

Das Problem mit solchen Runtime-Fehlern ist, das der Fehler nur dann auftritt, wenn das Programm auch den korrespondierenden Code ausführt.
Compiletime-Fehler, allerdings werden immer erkannt, auch wenn das Programm nicht den spezifischen Pfad wählt.

### Verwendung der Library

#### Declaration einer Tabelle als Struct in Rust

```rust
pub struct Module {
    pub id:     i64,
    pub name:   String,
    pub serial: String,
}
```

#### Verbinden mit Datenbank

**Wichtig**: Der Pfad der Datenbank ist in der Umgebungsvariable DATABASE_URL enthalten.
Der nachstehende Code ist **NICHT** funktionsfähig, da die SQLx Funktionsaufrufe impl Future<Output = _> rückgeben, und diese Future von einer Async-Runtime ausgeführt werden müssen.

```rust
let conn = SqlitePool::connect(env!("DATABASE_URL"))?,
```

#### Abfrage

```rust
let data: Vec<Module> = sqlx::query_as!(
    Module,
    "SELECT * FROM modules"
).fetch_all(&conn);
```

SQLx ist eine asynchrone Library, dazu ausgelegt um mit der Tokio Async Runtime Netzwerkanwendungen zu implementieren.
Die offizielle Egui Implementation unterstützt kein Async, daher werden die Abfragen synchron ausgeführt.

```rust
let rt = tokio::runtime::Runtime::new()?;
rt.block_on(future)?,
```


# Abkürzungen/Fremdwörter

## Dependency:
Library die ein Programm/Library benötigt um selbst zu funktionieren.

## Static Typing:
Datentypen werden zur Compiletime überprüft.

## Dynamic Typing:
Datentypen werden zur Runtime überprüft.

## Strong Typing:
Datentypen können nicht implizit in andere umgewandelt werden.

## Weak Typing:
Datentypen können implizit in andere umgewandelt werden.

## Borrowchecker:
Teil des Rust-Compilers, welcher die Gültigkeit von Referenzen überprüft.

## Referenz:
Ein Pointer, der zur Compiletime überprüft wird.

## Pointer:
Datentyp welcher eine Speicheraddresse und den Datentypen des Wertes bei besagter Addresse beinhaltet.

## Smart Pointer:
Pointer als Owned Type.

## Owned Type:
Primitive Datentypen werden in Structs umhüllt, um Verhalten wie zB automatisches Zerstören von Resourcen zur Verfügung zu stellen.

## Slice:
Datentype welcher aus einem Pointer und einer Länge besteht.

## Compiler:
Programm, welches eine Sprache (oft menschennahe Hochsprache) in ein anderes Format (oft Assembly) übersetzt.

## Assembly:
Menschenlesbares Format von Maschinencode.

## Thread:
Ausführungsfaden eines Prozesses.
Threads können als kleinere Prozesse innerhalb eines Prozesses angesehen werden.
Jeder Thread hat einen eigenen Ausführungskontext. (Stack / Register)

## Future:
Ein Konzept um das Arbeiten mit Asynchronen Funktionen zu erleichtern.
Eine Future ist ein Stück Arbeit die in der Zukunft erledigt wird. Eine Future kann "gepollt" werden, um einen kleinen Teil der Arbeit durchzuführen.
Es ist die Aufgabe einer Async-Funtime wie zB Tokio, diese Futures zu verwalten, und sie auszuführen.

## Struct:
Compound-Type, Datentyp zusammengestellt aus mehreren anderen Datentypen.

## Enum:
Tagged Union, kann verschiedene States annehmen.
Diese States können einen konstanten Index darstellen (usize), einen tuple, ein field, oder keinen spezifischen Wert.

## Tagged Union:
Union, bei dem ein Tag (enum) angibt welcher Member zurzeit aktiv ist.

## Union:
Ähnlich zu Struct, aber Compiler allokiert nur Speicher für den größten Member.

## Makro:
Feature in Programmiersprachen um Metaprogrammierung zu ermöglichen

## Metaprogrammierung:
Eine Programmiermethode, bei der Code in einem Programm, weiteren Code im selben Programm erzeugt

## Crate:
Translationunit in Rust, analog zu einem "Package" in anderen Sprachen

## SQL:
Structured Query Language, Abfragesprachen für Datenbanken

## DB:
Datenbank
