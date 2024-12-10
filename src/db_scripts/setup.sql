CREATE TABLE modules (
    id      INTEGER PRIMARY KEY,
    name    TEXT    NOT NULL UNIQUE,
    serial  TEXT    NOT NULL UNIQUE
);

-- Sollwerte
CREATE TABLE targetvalues (
    id         INTEGER PRIMARY KEY,
    module_id  INTEGER NOT NULL,
    identifier TEXT    NOT NULL,
    descriptor TEXT,
    value      DOUBLE  NOT NULL,
    unit       TEXT,
    FOREIGN KEY(module_id) REFERENCES modules(id)
);

-- Übungsblätter
CREATE TABLE documents (
    id         INTEGER PRIMARY KEY,
    module_id  INTEGER NOT NULL,
    document   BLOB    NOT NULL,
    descriptor TEXT NOT NULL,
    FOREIGN KEY(module_id) REFERENCES modules(id)
);

-- Schaltmatrix
CREATE TABLE matrix (
    id       INTEGER PRIMARY KEY,
    gnd      INTEGER NOT NULL,
    v_plus   INTEGER NOT NULL,
    v_minus  INTEGER NOT NULL,
    dds1_out INTEGER NOT NULL,
    dds2_out INTEGER NOT NULL,
    dds3_out INTEGER NOT NULL,
    adc_in   INTEGER NOT NULL
);
