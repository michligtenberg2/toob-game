# Battle of Culiacán RTS Game - El Culiacanazo

A real-time strategy game based on the events of October 17, 2019, in Culiacán, Mexico.

## 🎮 Game Overview

Experience the intense urban warfare that unfolded during the failed attempt to capture Ovidio Guzmán López. Play as the Sinaloa Cartel defending against government forces in this tactical RTS.

### ✨ Features

- **Isometric 2D Graphics**: Strategic top-down view with isometric perspective
- **Wave-Based Combat**: Progressive difficulty with increasingly challenging military waves  
- **Dual Faction Gameplay**: Control Cartel units defending against Military forces
- **Real-Time Strategy**: Tactical unit placement, combat, and resource management
- **Audio Enhancement**: Procedural sound effects and atmospheric radio chatter
- **Professional UI**: Real-time status displays, health bars, and mission objectives

## 🚀 Quick Start

### Prerequisites
- Rust (latest stable)
- Cargo

### Installation & Running
```bash
git clone https://github.com/michligtenberg2/toob-game.git
cd toob-game
cargo run
```

## 🎯 Gameplay

### Objective
Defend Ovidio Guzmán López and prevent his capture by government forces.

### Controls
- **SPACE**: Deploy Roadblock
- **R**: Call Reinforcements  
- **ESC**: Exit Game
- **F1**: Help

### Unit Types
- **🔫 Sicario**: Basic Cartel fighter
- **💪 Enforcer**: Heavy Cartel unit
- **👑 Ovidio**: High-value target (must protect)
- **🪖 Soldier**: Basic Military unit
- **⭐ Special Forces**: Elite Military unit  
- **🚗 Vehicle**: Armored Military transport
- **🚧 Roadblock**: Defensive barrier

## 🛠️ Technical Details

- **Engine**: Bevy 0.12 (Rust game engine)
- **Graphics**: 2D sprites with isometric transformation
- **Audio**: bevy_kira_audio integration with procedural effects
- **Architecture**: Entity Component System (ECS)

## 📚 Documentation

Detailed documentation is available in the `docs/` directory:

- [Graphics Upgrade](docs/GRAPHICS_UPGRADE.md) - Visual enhancements
- [Audio System](docs/AUDIO_SYSTEM.md) - Sound design and implementation  
- [Gameplay Mechanics](docs/GAMEPLAY.md) - Game rules and balance

## 🌟 Development Status

**Current Version**: Phase 4A - Audio Enhanced RTS
- ✅ Complete RTS mechanics
- ✅ Professional graphics and UI
- ✅ Isometric camera system
- ✅ Wave-based combat system
- ✅ Audio enhancement with procedural effects

## ⚖️ Disclaimer

This game is based on historical events for educational and entertainment purposes. It does not endorse or promote any illegal activities.

---

*Built with Rust 🦀 and Bevy 🕊️*
