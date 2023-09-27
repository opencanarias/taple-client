use env_logger::Env;
use taple_client::{
    settings::{client_settings_builder, ClientSettings, SettingsGenerator},
    ClientBuilder, EMPTY,
};
use tokio::signal;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let settings = &client_settings_builder().build();
    let settings = ClientSettings::generate(settings).expect("Settings created");

    let client = ClientBuilder::new().build(
        settings,
        EMPTY,
        None
    ).expect("Client built");

    client.bind_with_shutdown(signal::ctrl_c());

    let void_notification_handler = |_| {};

    client.run(void_notification_handler).await;
}
