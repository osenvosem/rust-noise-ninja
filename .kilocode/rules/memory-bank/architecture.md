# Noise Ninja Architecture

## System Overview

Noise Ninja is a cross-platform desktop/mobile application built with Tauri 2.0 and Leptos. The architecture follows a client-side rendered (CSR) pattern where the frontend is built with Leptos and communicates with the Tauri backend for system-level operations.

## Component Structure

### Frontend Components

-   [`src/main.rs`](src/main.rs:1) - Application entry point that mounts the Leptos app to the DOM
-   [`src/app.rs`](src/app.rs:1) - Main application component that manages global state and orchestrates all other components
-   [`src/shared.rs`](src/shared.rs:1) - Shared data structures, constants, and utility functions used across components

### UI Components

-   [`src/components/grid.rs`](src/components/grid.rs:1) - Grid-based sound arrangement interface
-   [`src/components/control_panel.rs`](src/components/control_panel.rs:1) - Playback controls (play/pause, volume, random mode)
-   [`src/components/sound_library.rs`](src/components/sound_library.rs:1) - Sound browsing and selection interface
-   [`src/components/presets.rs`](src/components/presets.rs:1) - Preset management system
-   [`src/components/schedule.rs`](src/components/schedule.rs:1) - Scheduling system for planned and recurring playback
-   [`src/components/settings_menu.rs`](src/components/settings_menu.rs:1) - Settings and grid configuration
-   [`src/components/button.rs`](src/components/button.rs:1) - Reusable button component

### Backend (Tauri)

-   [`src-tauri/src/lib.rs`](src-tauri/src/lib.rs:1) - Core backend library
-   [`src-tauri/src/main.rs`](src-tauri/src/main.rs:1) - Tauri application entry point
-   [`src-tauri/Cargo.toml`](src-tauri/Cargo.toml:1) - Backend dependencies and configuration
-   [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json:1) - Tauri configuration

## Data Flow Architecture

### State Management

The application uses Leptos' reactive signals for state management:

-   `grid_data` - Stores the current grid configuration with sound assignments
-   `play` - Controls playback state (playing/paused)
-   `volume` - Global volume control
-   `gap_duration` - Duration between sounds
-   `random_playback` - Determines if sounds play in sequence or randomly
-   `presets` - Collection of saved configurations
-   `schedules` - Planned and recurring playback schedules

### Data Persistence

-   Uses Tauri's store plugin for persistent storage in `store.bin`
-   Automatically saves/loads application state (grid, volume, gap duration, etc.)
-   Preset management with save/load/delete operations
-   Schedule persistence for planned and recurring schedules

### Sound Library Structure

-   Located in `public/sounds/` directory with categorized subdirectories
-   Sound metadata stored in `public/sounds/lib.json`
-   Supports multiple categories: boom, doors, people, construction, eerie
-   Each sound has ID, filename, filepath, category, and duration properties

## Critical Implementation Paths

### Audio Playback

1. Grid sounds are played sequentially or randomly based on `random_playback` setting
2. Audio elements are controlled via Web Audio APIs through Leptos bindings
3. Gap duration controls the pause between sounds
4. Two audio elements are used: one for main playback, one for preview in sound library

### Scheduling System

1. Two schedule types: Planned (one-time) and Recurring (weekly patterns)
2. Uses `chrono` for date/time calculations
3. Scheduled playback effect continuously checks current time against active schedules
4. Automatically loads appropriate preset and starts playback when schedule is active

### Mobile Responsiveness

-   Grid size adjusts based on touch capability detection
-   Mobile devices use 4-column grid, desktop uses 6-column grid
-   Responsive design adapts to different screen sizes

## Build System

-   Uses Trunk for building the frontend
-   Tauri handles cross-platform packaging
-   Supports building for Windows, macOS, Linux, and Android
-   Tailwind CSS for styling
