-- Add up migration script here
CREATE TABLE IF NOT EXISTS url_map (
    short_url VARCHAR(50) NOT NULL,
    long_url VARCHAR(200) NOT NULL,
    PRIMARY KEY(short_url)
);
