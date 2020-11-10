// @generated SignedSource<<a92060693c186f9ec9b8fe0dc52adcdd>>
// Generated by $ cargo run -p fixture-tests -- oss/crates/relay-transforms/tests/sort_selections

mod sort_selections;

use sort_selections::transform_fixture;
use fixture_tests::test_fixture;

#[test]
fn sort_selections_transform() {
    let input = include_str!("sort_selections/fixtures/sort-selections-transform.graphql");
    let expected = include_str!("sort_selections/fixtures/sort-selections-transform.expected");
    test_fixture(transform_fixture, "sort-selections-transform.graphql", "sort_selections/fixtures/sort-selections-transform.expected", input, expected);
}