DROP TRIGGER trigger_users_update_updated_at ON users;
DROP FUNCTION fn_update_updated_at();

DROP INDEX idx_users_name;
DROP INDEX idx_users_key;

DROP TABLE users;
