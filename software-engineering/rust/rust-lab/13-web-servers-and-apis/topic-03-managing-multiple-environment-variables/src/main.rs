/*

cargo add config

touch env

Add this to the file:

APP_TEST=from_file

_______________________________________________________________________________

You can also load settings from a `settings.toml` file

cargo add serde -F derive

Create a `.env` file

Add this to the file
APP_TEST=from_file

*/


use config::Config;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MyConfig {
    test_toml: String,
    test: String,
}

fn main() {
    let _ = dotenvy::dotenv();

    let settings_reader = Config::builder()
        .add_source(config::File::with_name("settings").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let settings = settings_reader
        .try_deserialize::<MyConfig>()
        .unwrap();

    println!("{settings:?}");
    // MyConfig { test_toml: "from_settings_dot_toml", test: "from_file" }
}
