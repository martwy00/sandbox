# Rust Powder Sandbox

A lightweight 2D cellular automata and falling-sand physics sandbox built in Rust with `eframe` and `egui`.

## Features
- **25 Interactive Elements:** Powders (Sand, Gunpowder, Salt), liquids (Water, Salt Water, Oil, Acid, Nitro), gases (Smoke, Steam, Fire), and solids (Metal, TNT, Fuse, Wood, Plant, Ice, Concrete, Glass, Rubber, Stone, Clay, Wall, Void).
- **Thermal Simulation:** Heat conduction through metal with dynamic color glow, melting, boiling, evaporation, and real-time temperature inspection (`°C` / `°F`) on hover.
- **Chain Reactions:** Explosive materials (`TNT`, `Gunpowder`, `Nitro`) with crater physics and fuses.
- **Tools & Controls:**
  - `Left Click` — Paint selected element
  - `Shift + Drag` — Draw straight lines
  - `Right / Middle Drag` — Pan camera
  - `Scroll` — Zoom towards cursor
  - `Ctrl + Scroll` — Resize brush
  - `Space` — Pause / Resume simulation
  - `C` — Clear canvas
