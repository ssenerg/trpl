use _11_automated_tests as pkg;

mod shared;

#[test]
fn it_adds_two() {
    shared::setup();
    assert_eq!(4, pkg::add(2, 2));
}
// cargo test --test integration_te
