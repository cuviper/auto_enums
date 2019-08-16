#![cfg(all(
    feature = "std",
    feature = "type_analysis",
    feature = "transpose_methods",
    feature = "try_trait",
    feature = "exact_size_is_empty",
    feature = "read_initializer",
))]

#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/auto_enum/*.rs");
    t.compile_fail("tests/ui/enum_derive/*.rs");
}
