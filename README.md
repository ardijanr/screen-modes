# Screen-Mode-Selector

Easy toggle between modes for two monitors.

Makes changing monitor layout simpler and faster.

Comes in handy when you have to present something or just want to enable your display.

Bind it to a key for optimal handyness.


![](readme_assets/2021-01-06_03-15.png)
### Standalone application

Depends on:
- [iced](https://github.com/hecrj/iced)
- xrandr



Opens a floating window that lets you click on what screen you want active and immediately closes after click/enter,
built entierly in rust, using the [iced GUI library](https://github.com/hecrj/iced).




TODO:
- Fix select with keyboard arrows/vim
- Nicer icons




Features:

- Only Primary
- Only Secondary
- True duplicate, highest common resolution)
- Extend (highest available on both monitors)

Future Features:
- Screen position left or right


# Installation/How to use

Compile or [download binary](https://github.com/ardijanr/Screen-Modes/releases/download/Beta/screen_modes) and place it in for example ~/.bin (create if necessary)


Example for keyboard shortcut binding in i3:

```
bindsym --release $mod+F2 exec --no-startup-id ~/.bin/screen_toggle
```

