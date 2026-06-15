-- Add migration script here
CREATE TABLE IF NOT EXISTS collections {
    id INTEGER PRIMARY KEY NOT NULL;
    name TEXT NOT NULL;
}