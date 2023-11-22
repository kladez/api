/// Get required environment variable.
///
/// # Panics
///
/// Panics if environment variable is not set and if value of environment variable is not Unicode.
///
/// # Examples
///
/// ```
/// # use kladez_api::helpers::require_env_var_string;
/// let value = "test value".to_string();
/// std::env::set_var("TEST", &value);
/// assert_eq!(require_env_var_string("TEST"), value);
/// ```
/// ```should_panic
/// # use kladez_api::helpers::require_env_var_string;
/// let non_unicode_value =
///     <std::ffi::OsString as std::os::unix::prelude::OsStringExt>::from_vec(vec![0x80]);
/// std::env::set_var("TEST", &non_unicode_value);
/// require_env_var_string("TEST");
/// ```
/// ```should_panic
/// # use kladez_api::helpers::require_env_var_string;
/// std::env::remove_var("TEST");
/// require_env_var_string("TEST");
/// ```
pub fn require_env_var_string(var_name: &str) -> String {
    use std::env::{
        var,
        VarError::{
            NotPresent,
            NotUnicode,
        },
    };

    var(var_name).unwrap_or_else(|err| match err {
        NotPresent => panic!("Environment variable `{var_name}` is not set."),
        NotUnicode(_) => panic!("Environment variable `{var_name}` is not Unicode."),
    })
}
