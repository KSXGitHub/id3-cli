use crate::error::Error;
use clap::ValueEnum;
use serde::Serialize;

/// Format of input/output text values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(about = "")]
pub enum TextFormat {
    Json,
    Yaml,
}

impl TextFormat {
    /// Convert a [serializable](Serialize) value to a string according to the specified [format](TextFormat).
    pub fn serialize(self, value: &impl Serialize) -> Result<String, Error> {
        match self {
            TextFormat::Json => serde_json::to_string_pretty(value).map_err(Error::from),
            TextFormat::Yaml => serde_yaml::to_string(value).map_err(Error::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TextFormat;
    use pretty_assertions::assert_eq;
    use serde::Serialize;
    use text_block_macros::text_block;

    #[derive(Debug, Serialize)]
    struct TestSubject {
        number: i32,
        string: &'static str,
        array: &'static [TestSubject],
    }

    const TEST_SUBJECT: TestSubject = TestSubject {
        number: 123,
        string: "text",
        array: &[
            TestSubject {
                number: 0,
                string: "a",
                array: &[],
            },
            TestSubject {
                number: 1,
                string: "a",
                array: &[TestSubject {
                    number: 0,
                    string: "b",
                    array: &[],
                }],
            },
        ],
    };

    #[test]
    fn json() {
        let received = TextFormat::Json
            .serialize(&TEST_SUBJECT)
            .expect("serialize test subject");
        let expected = text_block! {
            r#"{"#
            r#"  "number": 123,"#
            r#"  "string": "text","#
            r#"  "array": ["#
            r#"    {"#
            r#"      "number": 0,"#
            r#"      "string": "a","#
            r#"      "array": []"#
            r#"    },"#
            r#"    {"#
            r#"      "number": 1,"#
            r#"      "string": "a","#
            r#"      "array": ["#
            r#"        {"#
            r#"          "number": 0,"#
            r#"          "string": "b","#
            r#"          "array": []"#
            r#"        }"#
            r#"      ]"#
            r#"    }"#
            r#"  ]"#
            r#"}"#
        };
        eprintln!("RECEIVED:\n{received}");
        assert_eq!(received, expected);
    }

    #[test]
    fn yaml() {
        let received = TextFormat::Yaml
            .serialize(&TEST_SUBJECT)
            .expect("serialize test subject");
        let expected = text_block! {
            "number: 123"
            "string: text"
            "array:"
            "- number: 0"
            "  string: a"
            "  array: []"
            "- number: 1"
            "  string: a"
            "  array:"
            "  - number: 0"
            "    string: b"
            "    array: []"
            ""
        };
        eprintln!("RECEIVED:\n{received}");
        assert_eq!(received, expected);
    }
}
