If you and the people you want to play with own the game on Steam, installing the mod is optional.  
**Windows only, Linux support requires a full patch.**
You can simply edit the config file to enable developer mode which will unlock Steam P2P and WIP content:  
- Download the game on Steam, run it once and exit it from the main menu.
- Locale the config folder: `C:\Users\YOUR_USERNAME\AppData\Local\LemnisGate\Saved\Config\WindowsNoEditor`
- - If you having trouble finding the `AppData` folder, you can press `Win+R`, input `%localappdata%` in the new window, and press `Ok`. It will open the `AppData\Local` for you, you can continue from there.
- There should be a bunch of `.ini` files. Open the `Game.ini` with Notepad (you can do this through right-clicking it).
- Copy this text and paste it to file:
```
[/Script/LemnisGate.ConvergenceProjectSettings]
EnvironmentType=DEV
```
- Save and close it.
- Make sure the people you want to play with are on your Steam friends list, and that both you and they are **online** in Steam.

If you did everything correctly, the next time you start the game you will find a new `P2P (DEV)` item in the menu, that will allow you to play with friends, and a number of new maps.