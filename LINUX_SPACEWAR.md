
So, you want to run Lemnis Gate on Linux, but you want it to show up as Spacewar for Steam P2P to work?  
This won't be pretty.  

- You have to actually download Spacewar. There is a bunch of ways to do that, one of them is to navigate your browser to this page and click this link: [steam://install/480](steam://install/480). Hopefully, Steam will show you the installation prompt.
- Get to the `Spacewar` folder. You can do that by right-clicking the game in your library, click `Properties`, open `Installed Files` tab and click `Browse...` button
- Note the `SteamworksExample.exe`. If it's there, delete everything inside this folder
- Move the Lemnis Gate game contents to this folder. Apply the mod if you haven't already done that.
- Rename `LemnisGate.exe` to `SteamworksExample.exe`, make sure it is now located exactly where the original `SteamworksExample.exe` was before
- You may need to apply `Steam Play compatibility tool` in Spacewar's properties, usual Linux stuff
- Launch `Spacewar` from the Steam, on the Steam Friends page check if it shows that you're playing it

May the Torvalds save your soul if you then have to find out how to manually install `VC 2019 Redist` and `DirectX Jun 2010 Redist`