# Screen-Toggle
Screen Toggle

Easy toggle between modes.

Makes changing monitor layout simpler.


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

    screen positioning



Implementation Spesifics for set_mode() function

Only primary display...
Command:
xrandr --output PRIMARY --auto --output SECONDAIRY --off

Only Secondairy
Command:
xrandr --output PRIMARY --off --output SECONDAIRY --auto


Duplicate: (True duplication, meaning same resolution on both)

xrandr --output PRIMARY --mode COMMON-RESOLUTION --off --output SECONDAIRY --mode COMMON-RESOLUTION --same-as PRIMARY


Extend:
xrandr --ouput PRIMARY --auto --output SECONDAIRY --auto --left-of PRIMARY




Notes:

Generate arguments, then pass each one to command

