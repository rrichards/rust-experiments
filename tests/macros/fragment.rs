use std::path::Path;

use styled::{EmitForTest, EmitPlain, EmitResult, StyledFragment};
use styled_macros::frag;

fn plain(frag: &StyledFragment) -> EmitResult<String> {
    frag.emit_into_string(EmitPlain)
}

fn color(frag: &StyledFragment) -> EmitResult<String> {
    frag.emit_into_string(EmitForTest)
}

macro_rules! test_case {
    (( $($frag:tt)* ) => plain: $plain:tt => colored: $colored:tt) => {
        assert_eq!(&plain(&frag!($($frag)*))?, $plain);
        assert_eq!(&color(&frag!($($frag)*))?, $colored);
    };

    ($frag:tt => plain: $plain:tt => colored: $colored:tt) => {
        assert_eq!(&plain(&frag!($frag))?, $plain);
        assert_eq!(&color(&frag!($frag))?, $colored);
    };
}

struct Stringy {
    value: String,
}

impl Stringy {
    fn value(&self) -> &str {
        &self.value
    }
}

#[test]
fn test_line() -> EmitResult {
    let value = ("outer-value",);
    let stringy = Stringy {
        value: "Niko".to_string(),
    };

    test_case!( [Red: "hello"]
        => plain: "hello"
        => colored: "[Red:hello]" );

    test_case!( "hello"
        => plain: "hello"
        => colored: "[normal:hello]" );

    test_case!(([Red: "hello"] [Green: "world"])
        => plain: "helloworld"
        => colored: "[Red:hello][Green:world]" );

    test_case!(([Red: "hello"] value.0 [Green: "world"])
        => plain: "helloouter-valueworld"
        => colored: "[Red:hello][normal:outer-value][Green:world]" );

    test_case!(([Red: "hello"] stringy.value() [Green: "world"])
        => plain: "helloNikoworld"
        => colored: "[Red:hello][normal:Niko][Green:world]" );

    test_case!(([Red: "hello"] (1 + 1) [Green: "world"])
        => plain: "hello2world"
        => colored: "[Red:hello][normal:2][Green:world]" );

    Ok(())
}

#[test]
fn test_block() -> EmitResult {
    let value = ("value-1", "value-2");
    let stringy = Stringy {
        value: "Niko".to_string(),
    };

    test_case!( ( [Red: "hello"] ; [Green: "world"] )
        => plain: "hello\nworld"
        => colored: "[Red:hello]\n[Green:world]" );

    test_case!( ( "hello" ; "world" )
        => plain: "hello\nworld"
        => colored: "[normal:hello]\n[normal:world]" );

    test_case!(([Red: "hello"] [Green: "world"] ; [Red: "goodbye"] "world")
        => plain: "helloworld\ngoodbyeworld"
        => colored: "[Red:hello][Green:world]\n[Red:goodbye][normal:world]" );

    test_case!(([Red: "hello"] value.0 [Green: "world"] ; [Red: "goodbye"] value.1 [Green: "world"])
        => plain: "hellovalue-1world\ngoodbyevalue-2world"
        => colored: "[Red:hello][normal:value-1][Green:world]\n[Red:goodbye][normal:value-2][Green:world]" );

    // test_case!(([Red: "hello"] stringy.value() [Green: "world"])
    //     => plain: "helloNikoworld"
    //     => colored: "[Red:hello][normal:Niko][Green:world]" );

    // test_case!(([Red: "hello"] (1 + 1) [Green: "world"])
    //     => plain: "hello2world"
    //     => colored: "[Red:hello][normal:2][Green:world]" );

    Ok(())
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    let ui_tests = Path::new(env!("CRATE_UI_TESTS"));

    eprintln!("ui tests at: {:?}", ui_tests);
    eprintln!(
        "fail tests at: {:?}",
        ui_tests.join("fail/*.rs").display().to_string()
    );

    // t.pass("tests/ui/pass/*.rs");
    t.pass(ui_tests.join("pass/*.rs").display().to_string());
    t.compile_fail(ui_tests.join("fail/*.rs").display().to_string());
}
