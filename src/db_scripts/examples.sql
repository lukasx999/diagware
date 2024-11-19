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
