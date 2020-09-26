

pub mod models {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Default, Deserialize, Clone)]
    pub struct Data {
        pub name: String,
        pub id: usize,
    }

}
