# Screen Mode Selector

Easy toggle between modes for two monitors.

Makes changing monitor layout simpler and faster.

Comes in handy when you have to present something or just want to enable your display.

Bind it to a key for optimal handyness.


![](readme_assets/screenshot.png)
### Standalone application

Source code depends on:
- [iced](https://github.com/hecrj/iced)


Requires xrandr.


Opens a floating window that lets you click on what screen you want active and immediately closes after click/enter,
built entierly in rust, using the [iced GUI library](https://github.com/hecrj/iced).


Keybindings are bound to numbers 1 to 4:

1 - Primary Screen Only (eDP)
2 - First Secondary screen only
3 - Duplicate
4 - Extend using --auto command
q - Exit
Esc - Exit


Features:

- Display only on Primary monitor
- Display only on Secondary monitor
- Duplicate screens, (actual duplicate, highest common resolution)
- Extend (highest available on both monitors)

Future Features:
- Screen position left or right (defaults to left right now)
    For now either change the source code argument where it says "--left-off" to "--right-of" in the main.rs file and recompile.
    or complain about it and it might happen sooner...


# Installation/How to use

Compile or [download binary](https://github.com/ardijanr/screen-modes/releases/download/Beta/screen_mode) and place it in for example ~/.local/bin (create if necessary)


Example for keyboard shortcut binding in i3:

```
bindsym --release $mod+F2 exec --no-startup-id ~/.bin/screen_toggle
```


Example nixos install:
```

{ config, lib, pkgs, ... }:

let
  url = "https://github.com/ardijanr/screen-modes/releases/download/latest/screen_mode";
  sha256 = "1wcngmhif8csv6iv9wh2w3gkrgh4f0gb0vkmkgxhfp89kdhb4jx1";

  runtimeDeps = with pkgs; [
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

  screenMode = pkgs.stdenv.mkDerivation {
    pname = "screen_mode";
    version = "0.1.0";
    src = pkgs.fetchurl {
      url = url;
      sha256 = sha256;
    };

    buildInputs = runtimeDeps;

    dontUnpack = true;

    installPhase = ''
      mkdir -p $out/bin
      cp $src $out/bin/screen_mode
      chmod +x $out/bin/screen_mode
    '';

    makeWrapperArgs = [
      "--set" "LD_LIBRARY_PATH" "${lib.makeLibraryPath runtimeDeps}"
    ];

    nativeBuildInputs = [ pkgs.makeWrapper ];

    postFixup = ''
      wrapProgram $out/bin/screen_mode ''${makeWrapperArgs[@]}
    '';

    meta = with lib; {
      description = "Select how external monitors should behave";
      homepage = "https://github.com/ardijanr/screen-modes";
      license = licenses.mit;
      maintainers = with maintainers; [ "ardijanr" ];
    };
  };
in
{
  home.packages = [
    screenMode
  ];
}
```