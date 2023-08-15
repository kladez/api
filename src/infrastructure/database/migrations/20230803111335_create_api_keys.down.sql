DROP TRIGGER trigger_api_keys_check_valid_until ON api_keys;
DROP FUNCTION fn_check_valid_until();

DROP TRIGGER trigger_api_keys_update_updated_at ON api_keys;

DROP INDEX idx_api_keys_name_user_id;
DROP INDEX idx_api_keys_user_id;
DROP INDEX idx_api_keys_key;
DROP INDEX idx_api_keys_name;

DROP TABLE api_keys;
