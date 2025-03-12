# Multithreading

Während dem Durchführen von Diagnosen ist es wichtig, dass die Diagnose nebenläufig zum UI durchgeführt wird.
Würde die Diagnose synchron ausgeführt werden, so würde das Rendering und Input-Handling des UI's blockieren, bis die Diagnose vollendet ist.
Daher wird die Diagnose parallel zum UI in einem seperaten Thread ausgeführt.

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

Future: Ein Konzept um das Arbeiten mit Asynchronen Funktionen zu erleichtern.
Eine Future ist ein Stück Arbeit die in der Zukunft erledigt wird. Eine Future kann "gepollt" werden, um einen kleinen Teil der Arbeit durchzuführen.

Struct: Compound-Type, Datentyp zusammengestellt aus mehreren anderen Datentypen.

Enum: Tagged Union, kann verschiedene States annehmen. Diese States können einen konstanten Index darstellen (usize), einen tuple, ein field, oder keinen spezifischen Wert.

Tagged Union: Union, bei dem ein Tag (enum) angibt welcher Member zurzeit aktiv ist.

Union: Ähnlich zu Struct, aber Compiler allokiert nur Speicher für den größten Member.

Makro: Feature in Programmiersprachen um Metaprogrammierung zu ermöglichen

Metaprogrammierung: Eine Programmiermethode, bei der Code in einem Programm, weiteren Code im selben Programm erzeugt

Crate: Translationunit in Rust, analog zu einem "Package" in anderen Sprachen

SQL: Structured Query Language, Abfragesprachen für Datenbanken

DB: Datenbank
