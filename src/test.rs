/// Test clap parsing
#[test]
fn verify_app() {
    use clap::IntoApp;
    use crate::Opt;
    Opt::into_app().debug_assert()
}
