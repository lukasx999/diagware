INSERT INTO modules
    (name, serial)
VALUES
    ('SUM',   '214232'),
    ('PAMP',  '554839'),
    ('DIFF',  '838429'),
    ('ESP32', '09485546'),
    ('INTEG', '488404322');

INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES
    (1, 'R',            'on',     500.45, 'ohm'),
    (1, 'U',            'offset', 0.0001, 'volt'),
    (2, 'Z',            NULL,     500000, 'ohm'),
    (3, 'power_factor', NULL,     0.5,     NULL);

INSERT INTO matrix
    (gnd, v_plus, v_minus, dds_out1, dds_out2, dds_out3, adc_in1, adc_in2)
VALUES
    (0, 5, 9, 2, 3, 5, 9, 4),
    (2, 3, 6, 5, 6, 3, 5, 6),
    (0, 5, 9, 2, 3, 5, 9, 4);

INSERT INTO documents
    (module_id, document, descriptor)
VALUES
    (1, readfile('documents/pamp_datasheet.pdf'), 'datasheet.pdf'),
    (1, readfile('documents/pamp_script.pdf'),    'script.pdf');
