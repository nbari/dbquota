CREATE TABLE IF NOT EXISTS test (
    uuid CHAR(36) CHARACTER SET ascii,
    time timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY(uuid));

INSERT INTO test (uuid) VALUES(UUID());
-- repeat N times just to increase db size
INSERT INTO test (uuid) SELECT (uuid()) FROM test;
INSERT INTO test (uuid) SELECT (uuid()) FROM test;
