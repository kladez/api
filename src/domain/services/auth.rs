use argon2::{
    password_hash::{
        rand_core::OsRng,
        Error as Argon2Error,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use poem_openapi::payload::Json;

use crate::{
    application::dtos,
    domain::{
        models,
        repositories,
    },
    infrastructure::Infrastructure,
};

pub struct Auth {
    user_repository: repositories::User,
    api_key_repository: repositories::user::ApiKey,
}

impl Auth {
    pub fn new(infrastructure: &Infrastructure) -> Self {
        Self {
            user_repository: repositories::User::new(infrastructure),
            api_key_repository: repositories::user::ApiKey::new(infrastructure),
        }
    }

    pub fn get_secret_hash(password: &str) -> Result<String, dtos::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    pub fn validate_secret(
        password: &str,
        hash: &str,
    ) -> Result<(), dtos::Error> {
        let hash = PasswordHash::new(hash)?;
        Argon2::default().verify_password(password.as_bytes(), &hash)?;
        Ok(())
    }

    pub async fn auth(
        &self,
        auth: &dtos::auth::requests::Auth,
    ) -> Result<models::User, dtos::Error> {
        let user = self
            .user_repository
            .get_by_name(&auth.name)
            .await?;

        Self::validate_secret(&auth.password, &user.password_hash)?;

        Ok(user)
    }

    pub async fn auth_with_api_key(
        &self,
        api_key: &str,
    ) -> Result<models::User, dtos::Error> {
        // let api_keys = self.api_key_repository.get(None).await?;
        // let api_key = api_keys
        //     .into_iter()
        //     .find(|api_key_hashed| Self::validate_secret(api_key, &api_key_hashed.hash).is_ok());

        // match api_key {
        //     Some(api_key) => {
        //         let user = self
        //             .user_repository
        //             .get(&api_key.user_id)
        //             .await
        //             .map_err(dtos::Error::from)?;

        //         Ok(user)
        //     }
        //     None => Err(dtos::Error::from(Argon2Error::Password)),
        // }

        todo!()
    }
}
