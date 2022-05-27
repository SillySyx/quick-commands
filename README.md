# Requirements
The following things needs to be installed in your linux machine for this application to work.

* Gtk3
* libappindicator3
* gnome-shell-extension-appindicator (https://github.com/ubuntu/gnome-shell-extension-appindicator)

# Installation
Download the latest release and extract the file to your preferred location.  
Example: `~/.local/share/quick-commands`

# Configuration
Create file `~/.config/quick-commands/config.yaml` containing the following
```
text: optional text in system tray
icon: custom gtk icon, default is open-menu-symbolic
bell reminder: 300

sections:
  - groups:
    - name: grouped commands
      items:
      - name: test program
        command: gnome-text-editor

      - name: test command
        command: flatpak run org.mozilla.firefox

  - items:
    - name: test script
      command: bash
      args: /home/user/script.sh

notifications:
  - title: Break!
    text: Time to stretch
    time: 09:00

  - title: Lunch!
    text: Take your time
    time: 12:00
```

# Autostart
If you don't have a easy way to add binaries to autostart you can add the follow file `~/.config/autostart/quick-commands.desktop`
```
[Desktop Entry]
Type=Application
Name=Quick-commands
Description=Starts quick-commands app
Exec=/home/user/.local/share/quick-commands
```