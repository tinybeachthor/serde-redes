use serde_with_extras::{extras, Extras, EXTRAS_COMMENT_AFTER, EXTRAS_COMMENT_BEFORE};

#[test]
fn empty_extras_serialize() {
    let inner = String::from("string");
    let extras = Extras::new(inner, extras!());

    let data = serde_json::to_string(&extras).unwrap();
    insta::assert_snapshot!(data);
}

#[test]
fn extras_serialize() {
    let inner = String::from("string");
    let extras = Extras::new(
        inner,
        extras!(
            EXTRAS_COMMENT_BEFORE.to_string() => String::from("Hello"),
            EXTRAS_COMMENT_AFTER.to_string() => String::from("Goodbye"),
        ),
    );

    let data = serde_json::to_string(&extras).unwrap();
    insta::assert_snapshot!(data);
}
