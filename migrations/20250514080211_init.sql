-- Add migration script here
CREATE TABLE IF NOT EXISTS links (
    id INT AUTO_INCREMENT PRIMARY KEY,
    `key` VARCHAR(255) NOT NULL UNIQUE,
    url TEXT NOT NULL,
    clicks INT DEFAULT 0
);