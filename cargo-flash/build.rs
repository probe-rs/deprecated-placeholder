fn main() {
    panic!(
        r#"
!!!!!!!!!!!!!!!!!!!!!!!!
cargo-flash is now a part of probe-rs. To use it, you need to install probe-rs. You can do this by executing the following command:

$ cargo install probe-rs --features cli

Once installed, you can use the "cargo flash" command just like before, although there might be minor changes in the API. For more information, please refer to the probe-rs documentation at https://probe.rs/docs/tools/cargo-flash/.
!!!!!!!!!!!!!!!!!!!!!!!!
"#
    );
}
