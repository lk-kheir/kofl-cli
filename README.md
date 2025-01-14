# Kofl Project

Kofl (Arabic words means Lock) Project is a Rust-based application designed to manage passwords buit by a dev for devs.

### Prerequisites

- Rust (latest stable version)
- SQLite3

### Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/lk-kheir/kofl.git
    cd kofl
    ```

2. **Build the project**:
    ```sh
    cargo build
    ```

3. **Run the project**:
    ```sh
    cargo run -- add google google_pwd
    ```

## Roadmap

### High Priority (Urgent) 🔴

#### Security

- [x] Implement password complexity requirements and validation
- [x] Add timeout/auto-lock feature
- [x] Rate limiting for failed master password attempts
- [x] Two-factor authentication support

#### Core Features

- [x] Comprehensive logging system
- [x] Backup/restore functionality
- [x] Integration tests
- [x] Proper error recovery mechanisms


### Medium Priority 🟡

#### User Experience

- [x] Password strength meter
- [x] Password expiration notifications
- [x] Clipboard integration with auto-clear
- [x] Command aliases and tab completion

#### Data Management:

- [x] Categories/folders for password entries
- [x] Search functionality
- [x] Import/export capabilities
- [x] Password history tracking

#### Architecture:

- [x] Database migrations
- [x] Configuration management improvements
- [x] Transaction support
- [x] State management refactoring


### Low Priority 🟢

#### Advanced Features:
- [x] Browser integration
- [x] Secure password sharing
- [x] File attachment support
- [x] Emergency access protocol

#### Quality of Life:
- [x] Interactive mode
- [x] Configuration profiles
- [x] Bulk operations support
- [x] Plugin system


## Current Improvements in Progress

### Code Quality

- [x] Enhancing error handling with descriptive messages
- [x] Expanding test coverage
- [x] Implementing comprehensive logging
- [x] Code refactoring for maintainability

### Performance

- [x] Query optimization
- [x] Resource usage improvements
- [x] Caching implementation

### Documentation

- [x] API documentation
- [x] Usage examples
- [x] Development guidelines
- [x] API documentation


## Tech Stack

1. Core: Rust
2. Database: Sqlite3
3. Encryption: AES-256
4. Configuration: Toml



## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
