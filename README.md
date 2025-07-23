# Screen Mode Selector

Easy toggle between modes for two monitors for xrandr based systems.

Makes changing monitor layout simpler and faster.

Comes in handy when you have to present something or just want to enable your display.

Bind it to a key for optimal handyness.


![](readme_assets/screenshot.png)
### Standalone application

Source code depends on:
- [iced](https://github.com/hecrj/iced)

See flake for additional dependecies.


Opens a floating window that lets you click on what screen you want active and immediately closes after click/enter,
built entierly in rust, using the [iced GUI library](https://github.com/hecrj/iced).


# Keybindings

- Key(  1  ) --  Primary Screen Only (eDP)
- Key(  2  ) --  First Secondary screen only
- Key(  3  ) --  Duplicate
- Key(  4  ) --  Extend using --auto command
- Key(  Q  ) --  Exit
- Key(  Esc  ) --  Exit


# Features:

- Display only on Primary monitor
- Display only on Secondary monitor
- Duplicate screens, (actual duplicate, highest common resolution)
- Extend (highest available on both monitors)

Future Features:
- Screen position left or right (defaults to left at the moment)
    For now either change the source code argument where it says "--left-off" to "--right-of" in the main.rs file and recompile.
    or complain about it and I might change it...


# Installation/How to use

Compile or [download binary](https://github.com/ardijanr/screen-modes/releases/download/Beta/screen_mode) and place it in for example ~/.local/bin (create if necessary)


Example for keyboard shortcut binding in i3:

```
bindsym --release $mod+F2 exec --no-startup-id ~/.bin/screen_toggle
```


# Example nixos home manager install:
```
  home.packages = [
    inputs.screen-modes.packages.${pkgs.system}.default
  ];
```
