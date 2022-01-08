CREATE TABLE IF NOT EXISTS torrust_torrent_files (
    file_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    torrent_id INTEGER NOT NULL,
    number INTEGER NOT NULL,
    path VARCHAR(255) NOT NULL,
    length INTEGER NOT NULL,
    FOREIGN KEY(torrent_id) REFERENCES torrust_torrents(torrent_id)
)
