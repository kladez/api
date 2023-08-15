use crate::{
    application::dtos,
    domain::repositories,
    infrastructure::Infrastructure,
};

pub struct Summary {
    repository: repositories::Summary,
}

impl Summary {
    pub fn new(infrastructure: &Infrastructure) -> Self {
        let repository = repositories::Summary::new(infrastructure);
        Self { repository }
    }

    pub async fn get(&mut self) -> Result<dtos::summary::Summary, dtos::Error> {
        self.repository.get().await.map_err(dtos::Error::from)
    }
}
