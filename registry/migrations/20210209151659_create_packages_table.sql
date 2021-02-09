-- Add migration script here
CREATE TABLE packages(
	id uuid NOT NULL,
	PRIMARY KEY (id),
	name TEXT NOT NULL UNIQUE
)
