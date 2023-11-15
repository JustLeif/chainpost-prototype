CREATE TABLE IF NOT EXISTS users (
    uid VARCHAR(59) PRIMARY KEY,
    display_name VARCHAR(50),
    email VARCHAR(64),
    registration_timestamp INTEGER NOT NULL,
);

CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content VARCHAR(300) NOT NULL,
    uid VARCHAR(59) NOT NULL,
    creation_timestamp INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES users(uid)
);