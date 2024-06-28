fn main() {
    const TEMPLATE: &str = r#"
!!!!!!!!!!!!!!!!!!!!!!!!
`cargo-flash` is now a part of `probe-rs`. To install it, {shell}:

{install-command}

Once installed, you can use the "cargo flash" command just like before, although there might be minor changes in the API. For more information, please refer to the documentation at https://probe.rs/docs/tools/cargo-flash/.
!!!!!!!!!!!!!!!!!!!!!!!!
"#;

    let (shell, install_command) = match std::env::consts::OS {
        "windows" => ("run the following PowerShell command", "irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex"),
        "linux" | "macos" => ("run the following command", "curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh"),
        _ => ("see the installation instructions on the website:", "https://probe.rs/docs/getting-started/installation/"),
    };

    panic!(
        "{}",
        TEMPLATE
            .replace("{shell}", shell)
            .replace("{install-command}", install_command)
    );
}
