-- Add migration script here
CREATE TABLE IF NOT EXISTS audit (
    id INTEGER PRIMARY KEY,
    total_files INTEGER NOT NULL,
    total_records INTEGER NOT NULL,
    runtime_conf TEXT NOT NULL,
    failure_reason TEXT,
    successed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

