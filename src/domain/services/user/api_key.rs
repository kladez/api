use base64::{
    engine::general_purpose::STANDARD as base64,
    Engine as _,
};
use rand::{
    rngs::OsRng,
    RngCore,
};

use crate::{
    application::dtos,
    domain::{
        models,
        repositories,
        services,
    },
    infrastructure::Infrastructure,
};

pub struct ApiKey {
    repository: repositories::user::ApiKey,
}

impl ApiKey {
    pub fn new(infrastructure: &Infrastructure) -> Self {
        let repository = repositories::user::ApiKey::new(infrastructure);
        Self { repository }
    }

    pub async fn create(
        &self,
        payload: &dtos::user::api_key::requests::Create,
        user_id: &i32,
    ) -> Result<String, dtos::Error> {
        let mut api_key = vec![0u8; 96];
        OsRng.fill_bytes(&mut api_key);

        let api_key = base64.encode(api_key);

        self.repository.create(payload, &api_key, user_id).await?;

        Ok(api_key)
    }

    pub async fn get_all(
        &self,
        user_id: &i32,
    ) -> Result<Vec<dtos::user::api_key::responses::Get>, dtos::Error> {
        let api_keys = self.repository.get(user_id).await.map(|api_keys| {
            api_keys
                .into_iter()
                .map(dtos::user::api_key::responses::Get::from)
                .collect()
        })?;

        Ok(api_keys)
    }
}
