CREATE TABLE modules (
    id      INTEGER PRIMARY KEY,
    name    TEXT    NOT NULL UNIQUE,
    serial  TEXT    NOT NULL UNIQUE
);

CREATE TABLE targetvalues (
    id         INTEGER PRIMARY KEY,
    module_id  INTEGER,
    identifier TEXT    NOT NULL,
    descriptor TEXT,
    value      DOUBLE  NOT NULL,
    unit       TEXT,
    FOREIGN KEY(module_id) REFERENCES modules(id)
);


    -- MATRIX = Table("matrix", {
    --     "gnd":     SQLTypes.INT,  # Not-Connected value MUST always be first! - index 0
    --     "v_plus":  SQLTypes.INT,
    --     "v_minus": SQLTypes.INT,
    --     "sig_ch1": SQLTypes.INT,
    --     "sig_ch2": SQLTypes.INT,
    --     "sig_ch3": SQLTypes.INT,
    --     "adc_ch1": SQLTypes.INT,
    -- })
