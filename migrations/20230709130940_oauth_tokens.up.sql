CREATE TABLE tokens (
	id TEXT PRIMARY KEY,
	access_token TEXT NOT NULL,
	refresh_token TEXT NOT NULL,
	expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
	FOREIGN KEY (id) REFERENCES discord_accounts (id)
);