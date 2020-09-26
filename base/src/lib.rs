

pub mod models {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Default, Deserialize, Clone)]
    pub struct Data {
        pub value: i64,
        id: usize,
    }

    impl Data {
        pub fn new(value: i64, id: usize) -> Self {
            Data { value, id }
        }
    }
}
