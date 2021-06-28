-- Your SQL goes here
CREATE TABLE words (
    id INTEGER PRIMARY KEY NOT NULL,
    word TEXT NOT NULL,
    phonetic TEXT,
    definition TEXT,
    translation TEXT,
    pos TEXT,
    collins INTEGER,
    oxford INTEGER,
    tag TEXT,
    bnc INTEGER,
    frq INTEGER,
    exchange TEXT,
    sentences TEXT,
    audio TEXT
);