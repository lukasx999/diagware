INSERT INTO modules (name, serial) VALUES ('SUM',   '214232');
INSERT INTO modules (name, serial) VALUES ('PAMP',  '554839');
INSERT INTO modules (name, serial) VALUES ('DIFF',  '838429');
INSERT INTO modules (name, serial) VALUES ('ESP32', '09485546');
INSERT INTO modules (name, serial) VALUES ('INTEG', '488404322');


INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES (1, 'R', 'on', 500.45, 'ohm');

INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES (1, 'U', 'offset', 0.0001, 'volt');

INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES (2, 'Z', NULL, 500_000, 'ohm');

INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES (3, 'power_factor', NULL, 0.5, NULL);


INSERT INTO matrix
    (id, gnd, v_plus, v_minus, dds1_out, dds2_out, dds3_out, adc_in)
VALUES (1, 0, 5, 9, 2, 3, 5, 9);

INSERT INTO matrix
    (id, gnd, v_plus, v_minus, dds1_out, dds2_out, dds3_out, adc_in)
VALUES (2, 3, 3, 6, 4, 5, 6, 7);

INSERT INTO matrix
    (id, gnd, v_plus, v_minus, dds1_out, dds2_out, dds3_out, adc_in)
VALUES (3, 5, 1, 2, 3, 3, 5, 4);
