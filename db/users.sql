--INSERT INTO users (user_name, user_email, user_pass_hash) VALUES ('ab', 'ab', X'ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb');

DROP TABLE IF EXISTS 'users';

CREATE TABLE 'users' (
    'user_id'           INTEGER PRIMARY KEY AUTOINCREMENT,
    'user_name'         TEXT(20) NOT NULL UNIQUE,
    'user_email'        TEXT(255) NOT NULL,
    'user_pass_hash'    BLOB(32)  NOT NULL,
    'created_at'        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX 'users_username_index' ON 'users' ('user_name');

INSERT INTO 'users' ('user_name', 'user_email', 'user_pass_hash') VALUES ('admin', 'test@test.com', X'CA978112CA1BBDCAFAC231B39A23DC4DA786EFF8147C4E72B9807785AFEE48BB');
INSERT INTO 'users' ('user_name', 'user_email', 'user_pass_hash') VALUES ('a', 'test@test.com', X'CA978112CA1BBDCAFAC231B39A23DC4DA786EFF8147C4E72B9807785AFEE48BB');

DROP TABLE IF EXISTS 'login';

CREATE TABLE 'login' (
    'login_id'      INTEGER PRIMARY KEY AUTOINCREMENT,
    'user_id'       INTEGER NOT NULL,
    'login_time'    DATETIME NOT NULL,
    FOREIGN KEY ('user_id') REFERENCES 'users' ('user_id')
);

CREATE INDEX 'login_user_id' ON 'login' ('user_id');

DROP TABLE IF EXISTS 'tokens';

CREATE TABLE 'tokens' (
    'token_id'      INTEGER PRIMARY KEY AUTOINCREMENT,
    'user_id'       INTEGER NOT NULL,
    'token'         BLOB(32) NOT NULL,
    'token_id_rdm'  BLOB(16) NOT NULL,
    'desc'          TEXT(255) NOT NULL,
    'token_time'    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    'expired'       BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY ('user_id') REFERENCES 'users' ('user_id')
);

CREATE INDEX 'token_user_id_index' ON 'tokens' ('user_id');

INSERT INTO 'tokens' ('user_id', 'token') VALUES (1, X'CA978112CA1BBDCAFAC231B39A23DC4DA786EFF8147C4E72B9807785AFEE48BB');
INSERT INTO 'tokens' ('user_id', 'token') VALUES (2, X'CA978112CA1BBDCAFAC231B39A23DC4DA786EFF8147C4E72B9807785AFEE48BB');

DROP TABLE IF EXISTS 'cookies';

CREATE TABLE 'cookies' (
    'cookie_id'     INTEGER PRIMARY KEY AUTOINCREMENT,
    'user_id'       INTEGER NOT NULL UNIQUE,
    'cookie'        BLOB(32) NOT NULL,
    'cookie_time'   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    'expired'       BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY ('user_id') REFERENCES 'users' ('user_id')
);

Select * FROM 'tokens' LEFT JOIN 'users' ON tokens.user_id=users.user_id;