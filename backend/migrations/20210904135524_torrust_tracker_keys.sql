CREATE TABLE IF NOT EXISTS torrust_tracker_keys (
    key_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    key VARCHAR(32) NOT NULL,
    valid_until INT(10) NOT NULL,
    FOREIGN KEY(user_id) REFERENCES torrust_users(user_id)
)
