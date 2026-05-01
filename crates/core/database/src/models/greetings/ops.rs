use revolt_result::Result;

use crate::Greeting;

#[cfg(feature = "mongodb")]
mod mongodb;
mod reference;

#[async_trait]
pub trait AbstractGreetings: Sync + Send {
    /// Fetch the greeting for a given vertical, falling back to "default"
    async fn fetch_greeting(&self, vertical: Option<&str>) -> Result<Greeting>;
}
