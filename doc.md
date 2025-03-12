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



# Abkürzungen

Crate: Eine Translationunit in Rust, analog zu einem "Package" in anderen Sprachen
SQL: Structured Query Language, Abfragesprachen für Datenbanken
DB: Datenbank
