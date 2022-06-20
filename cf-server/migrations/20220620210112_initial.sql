-- Add migration script here
CREATE TABLE IF NOT EXISTS anilist_channels (
    anilist_channel_id INTEGER PRIMARY KEY,
    anilist_id INTEGER,
    status TEXT CHECK(status IN ('FINISHED', 'RELEASING', 'NOT_YET_RELEASED', 'CANCELED', 'HIATUS')) NOT NULL,
    entry_time TEXT
)
