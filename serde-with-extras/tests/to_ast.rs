use serde_with_extras::{extras, Extras, EXTRAS_COMMENT_AFTER, EXTRAS_COMMENT_BEFORE};

#[test]
fn empty_extras_to_ast() {
    let inner = String::from("string");
    let extras = Extras::new(inner, extras!());

    let data = serde_ast::to_ast(&extras).unwrap();
    insta::assert_snapshot!(data);
}

#[test]
fn extras_to_ast() {
    let inner = String::from("string");
    let extras = Extras::new(
        inner,
        extras!(
            EXTRAS_COMMENT_BEFORE => String::from("Hello"),
            EXTRAS_COMMENT_AFTER => String::from("Goodbye"),
        ),
    );

    let data = serde_ast::to_ast(&extras).unwrap();
    insta::assert_snapshot!(data);
}
