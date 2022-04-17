use gtk::prelude::*;
use gtk::{Menu, MenuItem};

use crate::command::execute_command;
use crate::configuration::{Configuration, Group, Item};

pub fn generate_menu(configuration: &Configuration) -> Menu {
    let menu = Menu::new();

    for section in &configuration.sections {
        let menu_items = generate_section(&section.groups);
        for menu_item in menu_items {
            menu.append(&menu_item);
        }

        let menu_items = generate_items(&section.items);
        for menu_item in menu_items {
            menu.append(&menu_item);
        }

        let separator = gtk::SeparatorMenuItem::new();
        menu.append(&separator);
    }

    let menu_item = gtk::MenuItem::with_label("Quit");
    menu_item.connect_activate(|_| {
        gtk::main_quit();
    });
    menu.append(&menu_item);

    menu
}

fn generate_section(groups: &[Group]) -> Vec<MenuItem> {
    let mut menu_items = Vec::new();

    for group in groups {
        let menu_item = MenuItem::with_label(&group.name);

        let submenu = Menu::new();
        let submenu_items = generate_items(&group.items);
        for item in submenu_items {
            submenu.append(&item);
        }

        menu_item.set_submenu(Some(&submenu));

        menu_items.push(menu_item);
    }

    menu_items
}

fn generate_items(items: &[Item]) -> Vec<MenuItem> {
    let mut menu_items = Vec::new();

    for item in items {
        let menu_item = MenuItem::with_label(&item.name);

        let command = item.command.clone();
        let args = item.args.clone();

        menu_item.connect_activate(move |_| {
            execute_command(&command, &args);
        });

        menu_items.push(menu_item);
    }

    menu_items
}