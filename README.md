# Screen-Toggle
Screen Toggle

Easy toggle between modes.

Makes changing monitor layout simpler.

![](readme_assets/2021-01-06_03-15.png)
### Standalone application

Depends on:

xrandr

X11


Opens a window that lets you click on what screen you want active and immediately closes after click/enter




TODO:
- Fix Floating on tiling window manager
- Fix close after click

- Fix select with keyboard arrows/vim
- Fix darker background color





1. Detect connected screens
     -- add them to a monitor struct

2. Make a window popup
     -- selection choices
         -- extend (highest available)
         -- duplicate (lowest common resolution)
         -- screen position left or right
         -- Only Secondary
         -- Only Primary


Future features

- screen positioning

