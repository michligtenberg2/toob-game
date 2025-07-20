You are a senior game developer and systems architect.

Design a fully installable real-time strategy (RTS) game — *not browser-based* — based exactly on the **Battle of Culiacán** (October 17, 2019), where Mexican armed forces attempted to arrest **Ovidio Guzmán López**, triggering an overwhelming counterattack by the Sinaloa Cartel.

This game must be a grounded, historical simulation in the style of *Command & Conquer*, with no fictionalization. The player controls the **cartel**, leveraging superior weapons and tactics against a numerically superior military force. The game should be cross-platform and desktop-ready (.exe, .app, or Linux AppImage).

---

### 🧱 Core Requirements:

#### 1. **Platform & Stack (Let AI decide)**
- You must choose the best programming language and framework for:
  - 2D RTS gameplay
  - Standalone desktop builds (no browser, no Unity license issues)
  - Performance and simplicity
- Examples: C++ + SDL, Rust + Bevy, Python + Pygame, MonoGame (C#), or anything installable

#### 2. **Game Structure**
- Top-down RTS, inspired by *Command & Conquer* layout
- Real map of **Culiacán** (low-poly city zones: Tres Ríos, military base, city center, airport)
- Real factions:
  - **Cartel**: playable — fewer units, better weapons, urban control
  - **Military**: AI — more units, official tactics, air and convoy units

#### 3. **Gameplay Mechanics**
- Asymmetric warfare systems:
  - Cartel: ambushes, roadblocks, drone jammers, hostage pressure
  - Military: armored vehicles, choppers, convoy extraction logic
- Urban panic: civilians, media attention, political pressure system
- Timed escalation: the longer the siege, the more unstable the city becomes

#### 4. **Mission Sequence (based on real timeline)**
- Mission 1: Secure Ovidio’s location
- Mission 2: Cut off military supply lines
- Mission 3: Apply pressure (hostages, threats to families)
- Mission 4: Hold until government relents and releases Ovidio

#### 5. **Code Output Goals**
- Generate a **recommended stack and file layout**
- Begin writing code for:
  - RTS-style unit and map controller
  - Spawn logic and AI movement
  - Event scripting (mission triggers, Ovidio capture/release)
- Emphasize modularity and performance: real-time loop, async tasking, low overhead

---

**Constraints:**
- Must be a standalone, installable desktop game
- No browser technologies, no Unity/Godot unless justified