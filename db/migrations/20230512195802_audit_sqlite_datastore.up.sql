-- Add migration script here
CREATE TABLE IF NOT EXISTS audit (
    id INTEGER PRIMARY KEY,
    user TEXT NOT NULL,
    hostname TEXT NOT NULL,
    total_files INTEGER NOT NULL,
    total_records INTEGER NOT NULL,
    failed_records INTEGER NOT NULL,
    record_failed_reason TEXT,
    runtime_conf TEXT NOT NULL,
    process_failure_reason TEXT,
    successed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);