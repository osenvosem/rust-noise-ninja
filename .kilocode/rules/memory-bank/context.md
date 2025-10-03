# Noise Ninja Context

## Current State

The Noise Ninja application is a fully functional cross-platform desktop/mobile application built with Tauri 2.0 and Leptos. It provides users with a comprehensive sound library for vibro-speakers to address noisy neighbors in a legal manner.

## Key Features Implemented

-   Grid-based sound arrangement interface with adjustable grid size (1-20 rows)
-   Sound library with 5 categories (Boom, Doors, People, Construction, Eerie) containing 67+ sounds
-   Preset management system for saving/loading configurations
-   Scheduling system with both planned and recurring playback options
-   Playback controls with volume adjustment and gap duration settings
-   Random playback mode for varied sound sequences
-   Mobile-responsive design that adapts to touch/non-touch devices

## Technical Implementation

-   Built with Rust using Leptos for frontend and Tauri 2.0 for backend
-   Client-side rendering with reactive state management
-   Persistent storage using Tauri's store plugin
-   Audio playback through Web Audio API with WASM bindings
-   Date/time scheduling using the chrono library

## Project Status

The application is feature-complete with all core functionality implemented. The codebase is organized with clear separation of concerns between UI components and shared utilities. The project is ready for cross-platform deployment to Windows, macOS, Linux, and Android.
