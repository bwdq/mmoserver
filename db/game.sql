DROP TABLE IF EXISTS 'characters';

CREATE TABLE 'characters'
(
    'char_id'       INTEGER PRIMARY KEY AUTOINCREMENT,
    'acct_id'       INTEGER NOT NULL,
    'char_name'     TEXT(20) NOT NULL UNIQUE,
    'json'          TEXT NOT NULL
);
