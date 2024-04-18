fn main() {
    panic!(
        r#"
!!!!!!!!!!!!!!!!!!!!!!!!
`probe-rs-cli` has been renamed to `probe-rs`. To use it, you need to install it by executing the following command:

$ cargo install probe-rs --features cli

For more information, please refer to the documentation at https://probe.rs/docs/tools/probe-rs/.
!!!!!!!!!!!!!!!!!!!!!!!!
"#
    );
}
