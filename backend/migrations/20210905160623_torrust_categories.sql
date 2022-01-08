CREATE TABLE torrust_categories (
    category_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(64) NOT NULL UNIQUE
);

INSERT INTO torrust_categories (name) VALUES
('movie'), ('game'), ('music'), ('tv show'), ('app'), ('documentary'), ('other');