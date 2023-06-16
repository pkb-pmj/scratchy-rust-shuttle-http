CREATE TABLE discord_accounts (id TEXT PRIMARY KEY);

CREATE TABLE scratch_accounts (
	username TEXT PRIMARY KEY,
	id TEXT NOT NULL,
	FOREIGN KEY (id) REFERENCES discord_accounts (id)
);

CREATE UNIQUE INDEX ON scratch_accounts (lower(username));