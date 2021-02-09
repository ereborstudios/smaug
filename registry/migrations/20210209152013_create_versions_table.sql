-- Add migration script here
CREATE TABLE versions(
	id uuid NOT NULL,
	PRIMARY KEY (id),
	package_id uuid NOT NULL REFERENCES packages(id) DEFERRABLE INITIALLY DEFERRED,
	version TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_versions_package_id_version ON versions(package_id, version);
