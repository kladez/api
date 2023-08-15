CREATE TABLE api_keys (
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	key VARCHAR(255) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT now(),
	updated_at TIMESTAMP NOT NULL DEFAULT now(),
	valid_until TIMESTAMP NOT NULL,
	deleted_at TIMESTAMP,
	user_id INTEGER NOT NULL REFERENCES users(id)
);

CREATE INDEX idx_api_keys_name ON api_keys(name);
CREATE INDEX idx_api_keys_key ON api_keys(key);
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);

CREATE UNIQUE INDEX idx_api_keys_name_user_id ON api_keys(name, user_id) WHERE deleted_at IS NULL;

CREATE TRIGGER trigger_api_keys_update_updated_at
BEFORE UPDATE ON api_keys
FOR EACH ROW
EXECUTE FUNCTION fn_update_updated_at();

CREATE OR REPLACE FUNCTION fn_check_valid_until() RETURNS TRIGGER AS $$
BEGIN
	IF (
		TG_OP = 'INSERT' OR
		TG_OP = 'UPDATE' AND NEW.valid_until IS DISTINCT FROM OLD.valid_until
	) AND NEW.valid_until <= now()
	THEN
		RAISE EXCEPTION 'valid_until must be in the future';
	END IF;
	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_api_keys_check_valid_until
BEFORE INSERT OR UPDATE ON api_keys
FOR EACH ROW
EXECUTE FUNCTION fn_check_valid_until();
