# Tech Context: Rust Noise Ninja

## Technologies Used

-   **Primary Language**: Rust (version 1.92.0-nightly)
-   **Framework**: Tauri (for cross-platform desktop application)
-   **Build System**: Cargo (Rust's package manager and build system)
-   **Frontend**: Web technologies (HTML, CSS, JavaScript) through Tauri
-   **Audio Processing**: Web Audio API or similar Rust audio libraries
-   **UI Components**: Custom components built with Rust and Tauri
-   **Configuration**: JSON configuration files for application settings

## Development Setup

-   **Rust Toolchain**: Using rust-toolchain file specifying version 1.92.0-nightly
-   **IDE**: VSCode with Rust extensions (rust-analyzer)
-   **Build Tools**: Trunk for web asset management, Cargo for Rust dependencies
-   **Package Management**: npm/yarn for frontend dependencies
-   **Version Control**: Git with GitHub repository at https://github.com/osenvosem/rust-noise-ninja.git

## Technical Constraints

-   Cross-platform compatibility (Windows, macOS, Linux)
-   Real-time audio processing without UI blocking
-   Efficient memory usage for audio playback
-   Responsive UI that works on different screen sizes
-   Secure handling of file system access for sound libraries
-   Performance optimization for pattern creation and playback
-   CPU architecture compatibility for development tools (Tauri dev command fails with "Bad CPU type in executable")

## Dependencies

-   Tauri framework dependencies
-   Rust audio processing libraries (likely cpal or similar)
-   Web asset bundling tools
-   Testing frameworks for Rust code
-   Development tools for debugging and profiling

## Tool Usage Patterns

-   Cargo for building, testing, and managing Rust dependencies
-   Trunk for web asset compilation and serving
-   npm/yarn for frontend package management
-   VSCode with rust-analyzer extension for development
-   Git for version control and collaboration
-   Tauri CLI for building desktop applications

## Architecture Decisions

-   Using Tauri instead of Electron for better performance and smaller binaries
-   Rust for core application logic due to memory safety and performance
-   Component-based UI architecture for maintainability
-   Asynchronous audio processing to prevent UI blocking
-   Categorized sound library structure for better organization

## Performance Considerations

-   Optimized audio loading and caching strategies
-   Efficient grid rendering for pattern visualization
-   Memory management for concurrent audio playback
-   Responsive design patterns for smooth UI interactions
-   Profiling tools integration for performance monitoring

## Updated Technical Context

Based on current project progress, the following technical aspects have been validated:

-   Tauri framework is successfully providing cross-platform support
-   Rust toolchain is properly configured with nightly version
-   Component-based architecture is working well for UI development
-   Audio processing approach is being implemented correctly
-   Build system (Cargo + Trunk) is functional for development

## Development Environment Status

-   **Rust Version**: 1.92.0-nightly (as specified in rust-toolchain)
-   **Tauri Status**: Fully integrated and configured
-   **Audio Libraries**: Being evaluated for integration (cpal or web audio API approach)
-   **UI Framework**: Tauri with Rust components working correctly
-   **Build Pipeline**: Cargo and Trunk configured and functional

## Future Technical Considerations

-   Audio library optimization for large sound collections
-   Advanced profiling tools integration
-   Memory usage monitoring during audio playback
-   Cross-platform performance testing
-   Scalability planning for future feature additions
