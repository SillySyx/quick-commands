mod configuration;
mod commands;
mod errors;

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};

use configuration::Configuration;
use commands::execute_command;

fn main() -> Result<(), &'static str> {
    let config = match Configuration::load_yaml_file() {
        Ok(config) => config,
        Err(error) => return Err(error.message),
    };

    gtk::init().unwrap();

    let mut menu = generate_menu_from_configuration_commands(&config);

    let mut indicator = AppIndicator::new("quick-commands appindicator", "");
    indicator.set_status(AppIndicatorStatus::Active);
    
    indicator.set_menu(&mut menu);
    
    menu.show_all();
    
    gtk::main();

    Ok(())
}

fn generate_menu_from_configuration_commands(configuration: &Configuration) -> gtk::Menu {
    let menu = gtk::Menu::new();

    for command in &configuration.commands {
        let name = command.name.to_owned();
        let args = command.args.to_owned();
        let command = command.command.to_owned();

        let menu_item = gtk::MenuItem::with_label(&name);
        menu_item.connect_activate(move |_| {
            execute_command(&command, &args);
        });
        
        menu.append(&menu_item);
    }

    let separator = gtk::SeparatorMenuItem::new();
    menu.append(&separator);

    let menu_item = gtk::MenuItem::with_label("Quit");
    menu_item.connect_activate(|_| {
        gtk::main_quit();
    });
    menu.append(&menu_item);

    menu
}