use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

// ([ ??? ]) => {
//     Json::Array(???)
// };
// ({ ??? }) => {
//     Json::Object(
//         ???
//     )
// };
// (???) => {
//     ??? // Number, String, Boolean
// };

macro_rules! json {
    (null) => { Json::Null };
    // Parses an array of strings into a Json::Array
    ([$($value:tt),*]) => {
        Json::Array(vec![$(json!($value)),*])
    };

    ({ $($key:tt: $value:tt),* }) => {
        {
            let mut map = HashMap::new();
            $(map.insert(String::from($key), json!($value));)*
            Json::Object(map)
        }
    };
    ($value:expr) => {
        Json::from($value)
     };
     

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_null_value() {
        assert_eq!(json!(null), Json::Null);
    }

    #[test]
    fn parse_a_valid_boolean() {
        assert_eq!(json!(true), Json::Boolean(true));
    }

    #[test]
    fn parse_a_valid_string() {
        assert_eq!(json!("Hello"), Json::String(String::from("Hello")));
    }

    #[test]
    fn parse_a_valid_array() {
        assert_eq!(
            json!(["a", "b", "c"]),
            Json::Array(vec![
                Json::String(String::from("a")),
                Json::String(String::from("b")),
                Json::String(String::from("c")),
            ])
        );
    }

    #[test]
    fn parse_a_valid_array_of_arrays() {
        assert_eq!(
            json!([["a", "b"], ["c"]]),
            Json::Array(vec![
                Json::Array(vec![
                    Json::String(String::from("a")),
                    Json::String(String::from("b"))
                ]),
                Json::Array(vec![Json::String(String::from("c"))]),
            ])
        );
    }

    #[test]
    fn parse_a_valid_object() {
        assert_eq!(
            json!({
                "Hello": "world",
                "Test": 1,
                "Names": [
                    "John",
                    "Doe"
                ]
            }),
            Json::Object(
                vec![
                    (String::from("Hello"), Json::String(String::from("world"))),
                    (String::from("Test"), Json::Number(1.0)),
                    (
                        String::from("Names"),
                        Json::Array(vec![
                            Json::String(String::from("John")),
                            Json::String(String::from("Doe")),
                        ])
                    ),
                ]
                .into_iter()
                .collect()
            )
        );
    }

    #[test]
    fn parse_a_valid_number() {
        assert_eq!(json!(1), Json::Number(1.0));
        assert_eq!(json!(1u8), Json::Number(1.0));
        assert_eq!(json!(1u16), Json::Number(1.0));
        assert_eq!(json!(1u32), Json::Number(1.0));
        assert_eq!(json!(1u64), Json::Number(1.0));
        assert_eq!(json!(1u128), Json::Number(1.0));
        assert_eq!(json!(1usize), Json::Number(1.0));
        assert_eq!(json!(1i8), Json::Number(1.0));
        assert_eq!(json!(1i16), Json::Number(1.0));
        assert_eq!(json!(1i32), Json::Number(1.0));
        assert_eq!(json!(1i64), Json::Number(1.0));
        assert_eq!(json!(1i128), Json::Number(1.0));
        assert_eq!(json!(1isize), Json::Number(1.0));
    }

    #[test]
    fn parse_a_valid_float() {
        assert_eq!(json!(1.0f32), Json::Number(1.0));
        assert_eq!(json!(1.0f64), Json::Number(1.0));
    }
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Boolean(value)
    }
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        Json::String(value)
    }
}

impl From<&str> for Json {
    fn from(value: &str) -> Self {
        Json::String(String::from(value))
    }
}

impl From<f64> for Json {
    fn from(value: f64) -> Self {
        Json::Number(value)
    }
}

impl From<f32> for Json {
    fn from(value: f32) -> Self {
        Json::Number(value as f64)
    }
}

impl From<u8> for Json {
    fn from(value: u8) -> Self {
        Json::Number(value as f64)
    }
}

impl From<u16> for Json {
    fn from(value: u16) -> Self {
        Json::Number(value as f64)
    }
}

impl From<u32> for Json {
    fn from(value: u32) -> Self {
        Json::Number(value as f64)
    }
}

impl From<u64> for Json {
    fn from(value: u64) -> Self {
        Json::Number(value as f64)
    }
}

impl From<u128> for Json {
    fn from(value: u128) -> Self {
        Json::Number(value as f64)
    }
}

impl From<usize> for Json {
    fn from(value: usize) -> Self {
        Json::Number(value as f64)
    }
}

impl From<i8> for Json {
    fn from(value: i8) -> Self {
        Json::Number(value as f64)
    }
}

impl From<i16> for Json {
    fn from(value: i16) -> Self {
        Json::Number(value as f64)
    }
}

impl From<i32> for Json {
    fn from(value: i32) -> Self {
        Json::Number(value as f64)
    }
}

impl From<i64> for Json {
    fn from(value: i64) -> Self {
        Json::Number(value as f64)
    }
}

impl From<i128> for Json {
    fn from(value: i128) -> Self {
        Json::Number(value as f64)
    }
}

impl From<isize> for Json {
    fn from(value: isize) -> Self {
        Json::Number(value as f64)
    }
}
