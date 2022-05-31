CREATE TABLE IF NOT EXISTS invites (
    invite_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    valid BOOLEAN NOT NULL,
    username VARCHAR(32),
    key VARCHAR(32) NOT NULL UNIQUE,
    FOREIGN KEY(username) REFERENCES torrust_users(username)
)
