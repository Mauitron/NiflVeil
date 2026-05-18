![niflveil-logo-complete](https://github.com/user-attachments/assets/74032954-8770-460d-87f0-c2057328197a)
# IMPORTANT: The eww interface is optional, I haven't improved upon the UI (might be buggy) as I don't use it myself. It is far more ergonomic to use the keybindings.

# NiflVeil
From the creator of [StygianSift](https://github.com/Mauitron/StygianSift) comes NiflVeil - a minimalistic window minimizer for Hyprland, named after Niflheim's mystical veil of mists where things vanish from sight but remain within reach. If you enjoy NiflVeil or want to see more tools from my workshop in Niflheim, consider buying me some [mead](https://buymeacoffee.com/charon0) 🍺

## Lost in the Mists? Let this paragon of sanity help you out!

Greetings, mortals! I am the keeper of the mists, formerly known as that boat guy from the warmer realms. After a... slight navigational error involving the world tree and some, exceptionally terrible, advice from an, exceptionally rude, squirrel, I've found myself in these frigid Norse lands. But fear not! I've turned my talent for ferrying lost souls into something more useful - helping you manage your lost windows!

## 🌫️ Features

### Core Features

- Window Veiling: Send your windows to Niflheim's mists, retrieve them when needed
- EWW Integration: To let you peer across the veil with style
- Waybar Support: Keep track of your hidden treasures
- Window State Persistence: Like Yggdrasil's roots, always remembering
- Window Previews: Capture the essence of your windows before they fade into the mists *(Currently under development)*


## 🛠️ Dependencies

- hyprland (your vessel through the desktop seas)
- eww (optional, but required for the restore menu interface)
- waybar (optional, but recommended for keeping track of your veiled windows)

## Installation

### 1. Building from source

```bash
# Clone the repository
git clone https://github.com/Mauitron/NiflVeil.git
cd NiflVeil/niflveil

# Build using cargo
cargo build --release

# Copy the binary to your path
sudo cp target/release/niflveil /usr/local/bin/

# If using EWW
# Create the niflveil widget folder in /etc/xdg/eww/widgets
mkdir /etc/xdg/eww/widgets/niflveil

# Copy eww contents to widgets folder
sudo cp eww/* /etc/xdg/eww/widgets/niflveil/ 
```

### 2. Add the bindings you want to your Hyprland config:

```bash
# Suggested NiflVeil bindings

# Minimize current window and updates the interface
bind = $mainMod, M, exec, /usr/local/bin/niflveil minimize


# Opens the EWW interface              
bind = $mainMod, I, exec, /usr/local/bin/niflveil restore

# Restore the last minimized window
bind = SUPER, U, exec, /usr/local/bin/niflveil restore-last

# Restore all minimized windows 
bind = $mainMod SHIFT, U, exec, /usr/local/bin/niflveil restore-all

# Close EWW interface with ESC key
bindn = , escape, exec, eww --config /etc/xdg/eww/widgets/niflveil/ close niflveil

# PS: If you are using the EWW window, add the following to the end
# of the bindings: "&& eww reload --config /etc/xdg/eww/widgets/niflveil"

```

### 3. (Optional) Add Waybar integration by adding this to your Waybar config:

```json
{
    "custom/niflveil": {
        "format": "{}",
        "exec": "niflveil show",
        "on-click": "niflveil restore",
        "return-type": "json",
        "interval": "once",
        "signal": 8
    }
}
```

And add this to your Waybar style.css:

```css
#custom-niflveil {
    padding: 0 10px;
    color: #88c0d0;
}

#custom-niflveil.empty {
    color: #4c566a;
}
```

## Usage

- Press Super + M to minimize the current window
- Press Super + I to show the restore menu via EWW
- Press Super + U to restore the most recently minimized window
- Press Super + Shift + U to restore all minimized windows

- Click the Waybar module (if configured) to show minimized windows

## Command Line Interface

```bash
niflveil [COMMAND] [WINDOW_ID]

Commands:
  minimize              Minimize the active window
  restore [window_id]   Restore a specific window or show restore menu
  restore-all           Restore all minimized windows
  restore-last          Restore the most recently minimized window
  show                  Display status for waybar integration
```

## Why Trust Your Windows to a Lost Ferryman?

Look, I might have taken a wrong turn at the River Styx and ended up in Niflheim, but I know a thing or two about managing lost things. Whether they're souls or windows, the principle is the same - keep track of them and make sure they can find their way back home!

## 🍺 Support the Development

If you find this tool useful, consider buying me some mead! I could use it in these cold Norse realms. Also, the heating bill in Niflheim is astronomical.

## 🌟 Acknowledgments

- The Hyprland community for not pointing out that I'm clearly lost
- Nordic theme creators for making me feel more at home in these cold lands
- That absolute tit Ratatoskr who gave me directions! (I should have known better)

P.S. No refunds, exchanges, or soul-backsies. The exchange rate between Obols and Norse currency is terrible anyway.

P.P.S. If you see a confused-looking boat anywhere in the world tree, please let me know. I'm starting to... yearn for it.





 
