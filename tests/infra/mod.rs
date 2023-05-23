use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[allow(unused)]
pub(crate) enum TestKind {
    Success,
    RuntimeError,
    StaticError,
}

#[macro_export]
macro_rules! success_tests {
    (subdir: $subdir:literal, $($tt:tt)*) => { $crate::tests!(Success, Some($subdir) => $($tt)*); };
    ($($tt:tt)*) => { $crate::tests!(Success, None => $($tt)*); }
}

#[macro_export]
macro_rules! runtime_error_tests {
    (subdir: $subdir:literal, $($tt:tt)*) => { $crate::tests!(RuntimeError, Some($subdir) => $($tt)*); };
    ($($tt:tt)*) => { $crate::tests!(RuntimeError, None => $($tt)*); }
}

#[macro_export]
macro_rules! static_error_tests {
    (subdir: $subdir:literal, $($tt:tt)*) => { $crate::tests!(StaticError, Some($subdir) => $($tt)*); };
    ($($tt:tt)*) => { $crate::tests!(StaticError, None => $($tt)*); }
}

#[macro_export]
macro_rules! tests {
    ($kind:ident, $subdir:expr =>
        $(
            {
                name: $name:ident,
                file: $file:literal,
                $(input: $input:literal,)?
                $(heap_size: $heap_size:literal,)?
                expected: $expected:literal $(,)?
                $(" $(tt:$tt)* ")?
            }
        ),*
        $(,)?
    ) => {
        $(
            #[test]
            fn $name() {
                #[allow(unused_assignments, unused_mut)]
                let mut input = None;
                $(input = Some($input);)?
                #[allow(unused_assignments, unused_mut)]
                let mut heap_size = None;
                $(heap_size = Some($heap_size);)?
                let kind = $crate::infra::TestKind::$kind;
                $crate::infra::run_test(stringify!($name), $subdir, $file, input, heap_size, $expected, kind);
            }
        )*
    };
}

pub(crate) fn run_test(
    _name: &str,
    subdir: Option<&str>,
    file: &str,
    _input: Option<&str>,
    _heap_size: Option<usize>,
    _expected: &str,
    _kind: TestKind,
) {
    let mut path = PathBuf::new();
    path.push("tests");
    if let Some(subdir) = subdir {
        path.push(subdir);
    }
    path.push(file);

    assert!(path.exists(), "{path:?} does not exist");
}
