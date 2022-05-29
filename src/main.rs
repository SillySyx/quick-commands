mod home_folder;
mod notifications;
mod bell;
mod indicator;
mod menu;
mod configuration;
mod command;
mod errors;

use gtk::prelude::*;

use configuration::Configuration;
use indicator::create_indicator;
use menu::generate_menu;

fn main() -> Result<(), &'static str> {
    gtk::init().unwrap();

    let config = match Configuration::load_yaml_file() {
        Ok(config) => config,
        Err(error) => return Err(error.message),
    };

    notifications::setup_notifications(&config);
    bell::setup_bell(&config);

    let mut indicator = create_indicator(&config);

    let mut menu = generate_menu(&config);

    indicator.set_menu(&mut menu);
    
    menu.show_all();
    
    gtk::main();

    Ok(())
}