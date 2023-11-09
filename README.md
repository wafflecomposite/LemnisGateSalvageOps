# Lemnis Gate: Salvage Ops
Lemnis Gate has been [discontinued since the April 11 2023](https://store.steampowered.com/news/app/950180/view/3678916525459103536), with the developers stating that the PC players will be completely unable to play.  
With all due respect, I'd like to disagree.  
### **The cycle must continue.**  

![Lemnis Gate: Salvage Ops](/Media/salvageops.png)
---

**Salvage Ops** is a mod set designed to bring the game back to a functional state and provide access to unreleased WIP content.  
Current scope is:  
- Allow **Peer-to-Peer multiplayer** through *Steam* and *LAN*, featuring *spectator* role
- Unlock **63 playable map prototypes** with **2 unreleased play modes**
- Unlock beta-only "Chameleon company" customization tree for every character
- Make it **playable on Steam Deck** (on Linux in general)

### Watch the Announcement Trailer on YouTube!
[![](https://markdown-videos.vercel.app/youtube/JrneJLFikWE)](https://youtu.be/JrneJLFikWE)

## Installation
### **IMPORTANT NOTE:**  If you **and** the people you want to play with own the game on **Steam**, installing the mod is **optional**. You can just *change one text file* to unlock Steam P2P and WIP content. [**Refer to this guide**](/EASY_P2P.md).

- Locate the game folder. For *Steam*, you can do that by right-clicking the game in your library, click `Properties`, open `Installed Files` tab and click `Browse...` button
- (Optional, Windows-only) If there is an `EasyAntiCheat` folder, uninstall it. Open the `EasyAntiCheat` folder, launch `EasyAntiCheat_Setup.exe`, click the `Uninstall` button in the left bottom corner in the EAC window, close the window, delete the `EasyAntiCheat` folder
- Download [`SalvageOps.zip` from the Releases tab](https://github.com/wafflecomposite/LemnisGateSalvageOps/releases/download/1.0/SalvageOps.zip)
- Move the contents of the archive to the folder of game (to the root one that contains `LemnisGate.exe`), agree to replace the files

Target game version: `1.3.26342`. We don't expect further updates, but if your Steam version of the game differs from this one, chances are the developers are intentionally trying to break this patch and changes may be required.
If your version of the game is different, please let me know about it through `Issues` tab at the top of the page.

## Usage
### Steam P2P:
- Launch the game through **Steam** (Make sure all players have launched the game with **Steam** and not by clicking the `.exe` files)
- Make sure everybody you want to play with is in your Steam Friend's list, is online and shown to be playing `Lemnis Gate`
- Click `P2P (DEV)` menu option
- To host a game, use `Create Session` ingame tab to create a match
- To join a game, use `Join Session` ingame tab to find the match. Inviting and joining through Steam overlay will **NOT** work
- Upon joining, match will be restarted

If at least one of the players does **not** have a game available in Steam:  
- This solution requires full mod installation, just changing the config file won't cut it
- Launch the game by clicking the `LemnisGate.exe` in game folder. (Make sure all players have launched the game through `LemnisGate.exe` and not through **Steam**)
- Make sure everybody is online in **Steam** and shown to be playing `Spacewar` (Spacewar's appID usage is allowed for developers' needs, see [Steamworks documentation](https://partner.steamgames.com/doc/sdk/api/example), there is no known cases of bans for that)
- Continue following the instructions above, starting with `P2P (DEV)` menu option
- It won't work on Linux. If you really desperate to run Lemnis Gate as Spacewar on Linux, there is a [special silly guide](/LINUX_SPACEWAR.md). If anyone knows the better way, please let me know.

### LAN P2P
- Does **NOT** require Steam at all
- This will work if you are physically on the same network, or connected to the same virtual private network together, by using tools like ZeroTier or Hamachi
- The firewall can be a problem, the host may need to disable it  

This will require full mod installed, and additional setup for everyone involved:
- Open the game folder
- From here, proceed to `Engine\Binaries\ThirdParty\Steamworks\Steamv147\Win64`
- Open the `DLL_LAN` folder, copy `steam_api64.dll`, get back and paste it to `Win64` folder, agree to replace the file
- (Should you change you mind and want to use Steam P2P again, do the same, but copy the `steam_api64.dll` from `DLL_STEAM` folder instead)
- From `Win64` folder, go to `lan_config\settings`, open the `account_name.txt` and enter the desired nickname
- Done. Continue following the instructions above, starting with `P2P (DEV)` menu option
- If you copied the game folder to another computer and can't find the lobby, while being sure that it's not a connection problem, having the same ID in "user_steam_id.txt" file might be the cause. Make sure to delete that file from the game's folder archive or change any few digits in it on the other's PCs so they differ. Check out the issue https://github.com/wafflecomposite/LemnisGateSalvageOps/issues/2 for more info.


## Known issues
- Inviting and joining through Steam overlay will **NOT** work, use the ingame `P2P (dev)` menu
- `EasyAntiCheat client integrity error` in main menu. (just press `Ok`)
- Modded game may open Frontier's issue tracker page in your default browser after the end of P2P match (you can just close it)
- LAN P2P match can sometimes show your PC device name for whatever reason. If that might be a problem for you, open the search bar, search for `about your pc`, open the settings and press the `Rename this PC` button  
- Some videos (usually on the loading screens) may not play on Steam Deck/Linux, instead showing a colorful table. This does not affect gameplay.  
- Occasional freezes if you finished a match, went to the menu, and tried to create a new lobby with a prototype map


'Lemnis Gate', 'Frontier', 'Frontier Foundry', Frontier logo and Lemnis Gate logo are trademarks of Frontier Developments plc and are not covered by this repository's license.
