use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_with_extras::{
    extras, to_with_extras, Extras, EXTRAS_COMMENT_AFTER, EXTRAS_COMMENT_BEFORE,
};

#[test]
fn extras_with_extras() {
    let inner = String::from("string");
    let extras = Extras::new(
        inner,
        extras!(
            EXTRAS_COMMENT_BEFORE.to_string() => String::from("Hello"),
            EXTRAS_COMMENT_AFTER.to_string() => String::from("Goodbye"),
        ),
    );

    let data = to_with_extras(&extras).expect("to_with_extras");
    insta::assert_snapshot!(data);
}

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
fn example_with_extras() {
    let example = Example {
        basic: "basic".to_string(),
        with_extras: "with_extras".to_string(),
    };
    let data = to_with_extras(&example).expect("to_with_extras");
    insta::assert_snapshot!(data);
}
