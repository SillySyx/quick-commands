use libappindicator::{AppIndicator, AppIndicatorStatus};

use crate::configuration::Configuration;

pub fn create_indicator(configuration: &Configuration) -> AppIndicator {
    let mut indicator = AppIndicator::new("quick-commands", "open-menu-symbolic");
    indicator.set_status(AppIndicatorStatus::Active);

    if let Some(text) = &configuration.text {
        indicator.set_label(text, "");
    }

    if let Some(icon) = &configuration.icon {
        indicator.set_icon(icon);
    }

    indicator
}