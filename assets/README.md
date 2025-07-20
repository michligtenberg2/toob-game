# Assets Directory

This directory contains all game assets organized by type.

## Directory Structure

```
assets/
├── audio/          # Background music and ambient sounds
├── sounds/         # Sound effects (gunfire, explosions, etc.)  
├── sprites/        # Unit sprites and textures
├── ui/            # User interface elements
├── maps/          # Map data and layouts
└── data/          # Game configuration files
```

## Asset Guidelines

### Audio Files
- Format: `.ogg` (preferred), `.wav`, `.mp3`
- Sample Rate: 44.1kHz or 48kHz
- Bit Depth: 16-bit minimum

### Sprites  
- Format: `.png` with transparency
- Resolution: Multiple of 16px for pixel-perfect rendering
- Color Space: sRGB

### Maps
- Format: `.json` or custom format
- Include spawn points, objectives, terrain data

## Current Implementation

The game currently uses:
- **Procedural sprites**: Colored rectangles with emoji overlays
- **Console audio**: Rich text-based sound descriptions
- **Placeholder assets**: Ready for real asset integration

## Future Assets Needed

- [ ] Unit sprite sheets
- [ ] Combat sound effects
- [ ] Background music tracks  
- [ ] UI textures and fonts
- [ ] Map backgrounds and tiles
