fn main() {
    panic!(
        r#"
!!!!!!!!!!!!!!!!!!!!!!!!
cargo-embed is now a part of probe-rs. To use it, you need to install it by executing the following command:

$ cargo install probe-rs --features cli

Once installed, you can use the "cargo embed" command just like before, although there might be minor changes in the API. For more information, please refer to the documentation at https://probe.rs/docs/tools/cargo-embed/.
!!!!!!!!!!!!!!!!!!!!!!!!
"#
    );
}
