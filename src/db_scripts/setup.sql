-- Module
CREATE TABLE modules (
    id      INTEGER PRIMARY KEY NOT NULL, -- NOT NULL, so SQLx wont require wrapping with Option<T>
    name    TEXT                NOT NULL UNIQUE,
    serial  TEXT                NOT NULL UNIQUE
);

-- Sollwerte
CREATE TABLE targetvalues (
    id         INTEGER PRIMARY KEY NOT NULL,
    module_id  INTEGER             NOT NULL,
    identifier TEXT                NOT NULL,
    descriptor TEXT,
    value      DOUBLE              NOT NULL,
    unit       TEXT,
    FOREIGN KEY (module_id) REFERENCES modules(id)
);

-- Übungsblätter
CREATE TABLE documents (
    id         INTEGER PRIMARY KEY NOT NULL,
    module_id  INTEGER NOT NULL,
    document   BLOB    NOT NULL,
    descriptor TEXT    NOT NULL,
    FOREIGN KEY(module_id) REFERENCES modules(id)
);

-- Schaltmatrix
CREATE TABLE matrix (
    id       INTEGER PRIMARY KEY NOT NULL,
    gnd      INTEGER NOT NULL,
    v_plus   INTEGER NOT NULL,
    v_minus  INTEGER NOT NULL,
    dds_out1 INTEGER NOT NULL,
    dds_out2 INTEGER NOT NULL,
    dds_out3 INTEGER NOT NULL,
    adc_in1  INTEGER NOT NULL,
    adc_in2  INTEGER NOT NULL
);
