# Windows Better Key
## Features
- When F13 key is pressed and held:
    - If another key is pressed, sends `Ctrl + <key>`.
    - If no other keys are pressed and F13 is released, sends `Esc`.
- Does not work for `F13 + Shift + <key>`. It would send `F13 + Shift` and `F13 + <key>` separately. However, `Shift + F13 + <key>` would work.

## Only works in Windows

## Remapping Caps Lock to F13
To use the Caps Lock key as F13:
1. Open Registry Editor:
   - Press `Win + R`, type `regedit`, and press Enter.
2. Navigate to: `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Keyboard Layout`.
3. Create a new binary value:
   - Right-click, select `New > Binary Value`, and name it `Scancode Map`.
4. Set the value to:
   ```
   00 00 00 00 00 00 00 00
   02 00 00 00 64 00 3A 00
   00 00 00 00
   ```
5. Restart your computer to apply the changes.
