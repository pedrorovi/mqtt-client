#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    ssid: &'static str,
    #[default("")]
    password: &'static str,
    #[default("")]
    host: &'static str,
    #[default(1883)]
    port: u16,
    #[default("")]
    client_id: &'static str,
    #[default("")]
    username: &'static str,
    #[default("")]
    mqtt_password: &'static str,
}

fn main() {
    println!("cargo:rerun-if-changed=cfg.toml");

    // Check if the `cfg.toml` file exists and has been filled out.
    if !std::path::Path::new("cfg.toml").exists() {
        panic!("You need to create a `cfg.toml` file with your Wi-Fi credentials! Use `cfg.toml.example` as a template.");
    }

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;
    if app_config.ssid == "" {
        panic!("You need to set the Wi-Fi credentials in `cfg.toml`!");
    } else {
        println!("Wi-Fi credentials are set in `cfg.toml`.");
    }

    embuild::espidf::sysenv::output();
}
