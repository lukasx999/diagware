INSERT INTO modules
    (name, serial)
VALUES ('SUM', '214232');

INSERT INTO modules
    (name, serial)
VALUES ('PAMP', '554839');

INSERT INTO modules
    (name, serial)
VALUES ('DIFF', '838429');

INSERT INTO modules
    (name, serial)
VALUES ('ESP32', '09485546');

INSERT INTO modules
    (name, serial)
VALUES ('INTEG', '488404322');



INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES (1, 'R', 'on', 500.45, 'ohm');

INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES (1, 'U', 'offset', 0.0001, 'volt');

INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES (2, 'Z', NULL, 500000, 'ohm');

INSERT INTO targetvalues
    (module_id, identifier, descriptor, value, unit)
VALUES (3, 'power_factor', NULL, 0.5, NULL);



INSERT INTO matrix
    (id, gnd, v_plus, v_minus, dds_out1, dds_out2, dds_out3, adc_in1, adc_in2)
VALUES (1, 0, 5, 9, 2, 3, 5, 9, 4);

INSERT INTO matrix
    (id, gnd, v_plus, v_minus, dds_out1, dds_out2, dds_out3, adc_in1, adc_in2)
VALUES (2, 2, 3, 6, 5, 6, 3, 5, 6);

INSERT INTO matrix
    (id, gnd, v_plus, v_minus, dds_out1, dds_out2, dds_out3, adc_in1, adc_in2)
VALUES (3, 0, 5, 9, 2, 3, 5, 9, 4);



INSERT INTO documents
    (module_id, document, descriptor)
VALUES (1, readfile('documents/pamp_datasheet.pdf'), 'datasheet.pdf');

INSERT INTO documents
    (module_id, document, descriptor)
VALUES (1, readfile('documents/pamp_script.pdf'), 'script.pdf');
