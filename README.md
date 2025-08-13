# A Graphical Solution to your Wayland Wallpaper Woes

![gif demo](assets/Gif.gif)
![random demo](assets/switch.gif)

## Dependencies
+ [swww](https://github.com/LGFae/swww)
+ GTK4
+ Up to date rustc compiler and cargo

## Build
### Automatic
+ [AUR](https://aur.archlinux.org/packages/gswww-git)
### Manual
To build, clone this repo:
```
git clone https://github.com/Ph1lll/Gswww.git && cd Gswww/
```
and inside run:
```
cargo build --release
```

Then put `./target/release/`  into your path,
and copy Gswww.desktop and Gswww.png with these commands:
 ```
cp assets/intergration/Gswww.desktop ~/.local/share/applications/
cp assets/intergration/Gswww-open.desktop ~/.local/share/applications/
cp assets/intergration/Gswww.png ~/.icons/
```

*Voilà,* now you can graphically change your wallpaper using the awesome swww deamon.
Just select the folder that holds your wallpapers, and click on the previews.

**Just make sure you actually have the daemon running otherwise you're gonna see nothing**

E.g. Hyprland
```
# ~/.config/hypr/hyprland.conf
exec-once = swww-daemon
```

## Features of swww (The daemon)
+ Display animated gifs as your wallpaper
+ Display images in any format (Some of these I didn't know existed)
    + jpeg
    + png
    + gif
    + tga
    + tiff
    + webp
    + pnm
    + bmp
    + farbfeld
+ Smooth transition effect when you switch images
+ Doing it all without having to pkill the daemon

## Big Thanks
Thanks goes to [LGFae](https://github.com/LGFae) for making [swww](https://github.com/LGFae/swww)
