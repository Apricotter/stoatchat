use revolt_result::Result;

use crate::Greeting;
use crate::MongoDb;

use super::AbstractGreetings;

static COL: &str = "greetings";

#[async_trait]
impl AbstractGreetings for MongoDb {
    async fn fetch_greeting(&self, vertical: Option<&str>) -> Result<Greeting> {
        if let Some(v) = vertical {
            let result: Option<Greeting> = self
                .col::<Greeting>(COL)
                .find_one(doc! { "_id": v })
                .await
                .map_err(|_| create_database_error!("find_one", COL))?;
            if let Some(g) = result {
                return Ok(g);
            }
        }

        self.col::<Greeting>(COL)
            .find_one(doc! { "_id": "default" })
            .await
            .map_err(|_| create_database_error!("find_one", COL))?
            .ok_or_else(|| create_error!(NotFound))
    }
}
