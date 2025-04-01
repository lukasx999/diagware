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
    module_id  INTEGER             NOT NULL,
    document   BLOB                NOT NULL,
    descriptor TEXT                NOT NULL,
    FOREIGN KEY(module_id) REFERENCES modules(id)
);

-- Schaltmatrix
CREATE TABLE matrix (
    id            INTEGER PRIMARY KEY NOT NULL,
    module_id     INTEGER             NOT NULL,
    gnd           INT                 NOT NULL,
    v_plus        INT                 NOT NULL,
    v_minus       INT                 NOT NULL,
    dds_out_plus  INT                 NOT NULL,
    dds_out_minus INT                 NOT NULL,
    v3_3          INT                 NOT NULL,
    adc_in1       INT                 NOT NULL,
    adc_in2       INT                 NOT NULL,
    FOREIGN KEY (module_id) REFERENCES modules(id)
);
