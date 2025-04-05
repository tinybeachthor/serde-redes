use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_with_extras::{extras, Extras, EXTRAS_COMMENT_AFTER, EXTRAS_COMMENT_BEFORE};

#[derive(Deserialize)]
struct Example {
    basic: String,
    with_extras: String,
}

impl Serialize for Example {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Example", 2)?;
        s.serialize_field("basic", &self.basic)?;
        let with_extras = Extras::new(
            &self.with_extras,
            extras!(
                EXTRAS_COMMENT_BEFORE.to_string() => String::from("Hello"),
                EXTRAS_COMMENT_AFTER.to_string() => String::from("Goodbye"),
            ),
        );
        s.serialize_field("with_extras", &with_extras)?;
        s.end()
    }
}

#[test]
fn serialize() {
    let example = Example {
        basic: "basic".to_string(),
        with_extras: "with_extras".to_string(),
    };
    let data = serde_json::to_string(&example).unwrap();
    insta::assert_snapshot!(data);
}
