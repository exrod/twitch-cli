
<<<<<<< HEAD

=======
>>>>>>> f5d9c85 (initial commit)
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub username: String,
    pub client_id: String,
    pub access_token: String,
}
