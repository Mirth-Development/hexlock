# HexLock

**A lockpicking "roguelike" : made for Wheel Jam 2, 2026**

Developed and lovingly rushed by **Gabe Allison** and **Kira Stephenson**.

There's no actual wheel in this game as we were told we could do a lock-picking game. We regret nothing. Maybe the real "wheel" was the friends we made along the way.

---

## Overview

Pick the lock against the clock. Send the lock's tumblers upward with your picks and catch them at the top to set them. Set every tumbler, in the correct order and before time runs out, to progress to the next lock.

There are no win/lose screens. Progression is capped: after **level 10** you're returned to the main menu.

---

## Controls

| Input | Action |
|---|---|
| `W` | Activate the **electric pick** (yellow) — hold to charge, used to stun set tumblers |
| `A` / `D` | Move the pick left / right |
| Arrow Keys *(or `J` `K` `L` `I`)* | Mini arrow-game input used to activate the **magic pick** (purple) |
| `Space` | Catch tumblers at the top of the lock to set them |
| `E` / `Q` | Cycle between lock pick types (3 total) |

### Pick types
- **Regular (white)** — smack regular tumblers in directly
- **Electric (yellow)** — hold `W` to charge; can stun already-set tumblers
- **Magic (purple)** — triggered via the arrow-key mini-game

---

## Tips

- Use the electric pick to stun and reset tumblers that are causing trouble.
- Tumblers must be set in **sequential order**, setting them out of order won't hold.
- If left unattended too long, tumblers fall back out and can knock loose higher-order tumblers.
- Rust build-up can be knocked off by striking the normal pick against the lock
- You'll be booted to the main menu after clearing level 10.

---

## Known Issues / Dev Notes

- The current build pulls in more Bevy crates than strictly necessary, inflating file size beyond what the game needs.
- No native Linux binary yet, Linux users can run the Windows build via **Wine** in the meantime.
- We're considering putting more time into this project post-jam to keep sharpening our Bevy skills.
