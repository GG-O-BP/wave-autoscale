CREATE TABLE metric (
  db_id TEXT PRIMARY KEY,
  id TEXT UNIQUE,
  collector TEXT,
  metadata TEXT,
  created_at timestamptz,
  updated_at timestamptz
);