# Noise Ninja Technology Stack

## Frontend Technologies

-   **Leptos** (0.8.0) - Reactive web framework for Rust

    -   Client-Side Rendering (CSR) mode
    -   Nightly features enabled
    -   Used for building the entire UI and managing state

-   **Rust** - Core programming language

    -   Used for both frontend and backend development
    -   Provides memory safety and performance

-   **WebAssembly (WASM)** - Target platform for Rust frontend
-   Compiled Rust code runs in browser environment
-   Provides near-native performance

## Backend Technologies

-   **Tauri 2.0** (2.8.5) - Cross-platform application framework

    -   Provides system-level access for desktop/mobile apps
    -   Handles native OS integration
    -   Bundles web frontend with native application

-   **Tauri Plugin Store** (2.4.0) - Persistent storage solution
    -   Used for saving application state
    -   Handles presets and schedule persistence

## Build Tools

-   **Trunk** - Build tool for Rust WASM applications
    -   Handles compilation of Rust to WASM
-   Manages frontend assets and dependencies

-   **Cargo** - Rust package manager and build system
    -   Manages project dependencies
    -   Handles compilation and testing

## UI and Styling

-   **Tailwind CSS** - Utility-first CSS framework

    -   Provides responsive design capabilities
    -   Used for styling the application

-   **Leptos Heroicons** - Icon library
    -   Provides UI icons for the application
    -   Integrated with Leptos framework

## Audio Handling

-   **Web Audio API** - Browser-native audio processing

    -   Handles playback of sound files
    -   Managed through Leptos bindings

-   **HTML5 Audio Elements** - Audio playback components
    -   Two audio elements used: main playback and preview
    -   Controlled via Rust WASM bindings

## Date/Time Handling

-   **Chrono** (0.4) - Date and time manipulation library
    -   Used for scheduling system
    -   Handles time calculations and timezone management

## Data Serialization

-   **Serde** - Serialization framework

    -   Handles JSON serialization/deserialization
    -   Used for data persistence and API communication

-   **Serde JSON** - JSON-specific serialization
    -   Used for storing application state
    -   Handles preset and schedule data

## Randomization

-   **Rand** (0.9) - Random number generation
    -   Used for random playback mode
    -   Generates unique IDs for presets and schedules

## Web Technologies

-   **WASM Bindgen** - WebAssembly bindings

    -   Enables Rust to interact with JavaScript APIs
    -   Provides access to browser features

-   **Web-sys** - Web platform bindings
    -   Provides access to browser APIs like Audio, Window, Navigator
-   Used for DOM manipulation and audio control

## Utilities

-   **Console Error Panic Hook** - Error handling

    -   Provides better error messages in browser console
    -   Helps with debugging in WASM environment

-   **Leptos-use** - Collection of useful Leptos utilities
    -   Provides timestamp functionality for scheduling
    -   Includes various helpful hooks

## Dependencies Summary

-   Frontend: Leptos (CSR), WASM
-   Backend: Tauri 2.0
-   Build: Trunk, Cargo
-   UI: Tailwind CSS, Leptos Heroicons
-   Data: Serde, Chrono
-   Audio: Web Audio API
-   Storage: Tauri Store Plugin
