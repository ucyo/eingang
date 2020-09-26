

pub mod models {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Default, Deserialize, Clone, Copy)]
    pub struct Data {
        pub value: i64,
        pub id: usize,  // Serde does not serialize if element is not public
    }

    impl Data {
        pub fn new(value: i64, id: usize) -> Self {
            Data { value, id }
        }
        pub fn update(&mut self, value: i64) {
            self.value = value
        }
    }

    use std::ops::{SubAssign, AddAssign};
    impl SubAssign<i64> for Data {
        fn sub_assign(&mut self, rhs: i64) {
            self.value -= rhs;
        }
    }
    impl AddAssign<i64> for Data {
        fn add_assign(&mut self, rhs: i64) {
            self.value += rhs
        }
    }
    use std::fmt::{Formatter, Result, Display};
    impl Display for Data {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Data(value={}, id={})", self.value, self.id)
        }
    }
    impl PartialEq<i64> for Data {
        fn eq(&self, other: &i64) -> bool {
            self.value == *other
        }
    }
    impl PartialEq<Data> for i64 {
        fn eq(&self, other: &Data) -> bool {
            *self == other.value
        }
    }
}
