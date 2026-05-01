use revolt_result::Result;

use crate::Greeting;
use crate::ReferenceDb;

use super::AbstractGreetings;

#[async_trait]
impl AbstractGreetings for ReferenceDb {
    async fn fetch_greeting(&self, _vertical: Option<&str>) -> Result<Greeting> {
        Ok(Greeting {
            vertical: "default".to_string(),
            message: "Hey {username}! Welcome to your studio.".to_string(),
        })
    }
}
