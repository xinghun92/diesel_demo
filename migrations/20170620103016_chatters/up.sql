CREATE TABLE chatters (
  id BIGINT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  type_ INTEGER NOT NULL,
  avatar_key TEXT NOT NUll DEFAULT '',
  update_time BIGINT NOT NULL DEFAULT 0,
  name_pinyin TEXT NOT NULL,
  creator_id BIGINT,
  is_resigned BOOLEAN
);

-- Your SQL goes here