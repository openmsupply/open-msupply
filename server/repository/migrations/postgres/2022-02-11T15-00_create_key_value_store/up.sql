-- https://github.com/openmsupply/open-msupply/blob/611f59207bd46178844c99c267f75115a721cc1c/server/repository/src/db_diesel/key_value_store.rs#L21-L40
CREATE TYPE key_type AS ENUM (
    -- Cursor for pulling central records from the central server
    'CENTRAL_SYNC_PULL_CURSOR',
    'REMOTE_SYNC_INITILISATION_STARTED',
    'REMOTE_SYNC_INITILISATION_FINISHED',
    'REMOTE_SYNC_PUSH_CURSOR',
    -- sync settings
    'SETTINGS_SYNC_URL',
    'SETTINGS_SYNC_USERNAME',
    'SETTINGS_SYNC_PASSWORD_SHA256',
    'SETTINGS_SYNC_INTERVAL_SEC',
    'SETTINGS_SYNC_CENTRAL_SERVER_SITE_ID',
    'SETTINGS_SYNC_SITE_ID',
    'SETTINGS_SYNC_SITE_UUID',
    'SETTINGS_SYNC_IS_DISABLED',
    --Used to validate JWT Tokens
    'SETTINGS_TOKEN_SECRET'
);

-- key value store, e.g. to store local server state
CREATE TABLE
    key_value_store (
        id key_type NOT NULL PRIMARY KEY,
        value_string TEXT,
        value_int INTEGER,
        value_bigint BIGINT,
        value_float DOUBLE PRECISION,
        value_bool BOOLEAN
    )
