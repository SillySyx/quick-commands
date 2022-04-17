# Requirements
The following things needs to be installed in your linux machine for this application to work.

* Gtk
* libappindicator
* gnome-shell-extension-appindicator (https://github.com/ubuntu/gnome-shell-extension-appindicator)

# Installation
Download the latest release and extract the file to your preferred location.  
Example: `~/.local/share/quick-commands`

# Configuration
Create file `~/.config/quick-commands/config.yaml` containing the following
```
commands:
  - name: Texty
    command: gnome-text-editor

  - name: Webby
    command: flatpak run org.mozilla.firefox

  - name: Scripty
    command: bash
    args: /home/user/script.sh
```

# Autostart
If you don't have a easy way to add binaries to autostart you can add the follow file `~/.config/autostart/quick-commands.desktop`
```
[Desktop Entry]
Type=Application
Name=Quick-commands
Description=Starts quick-commands app
Exec=/home/user/.local/share/quick-commands/quick-commands
```