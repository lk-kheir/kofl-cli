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

- [ ] Implement password complexity requirements and validation
- [ ] Add timeout/auto-lock feature
- [ ] Rate limiting for failed master password attempts
- [ ] Two-factor authentication support

#### Core Features

- [ ] Comprehensive logging system
- [ ] Backup/restore functionality
- [ ] Integration tests
- [ ] Proper error recovery mechanisms


### Medium Priority 🟡

#### User Experience

- [ ] Password strength meter
- [ ] Password expiration notifications
- [ ] Clipboard integration with auto-clear
- [ ] Command aliases and tab completion

#### Data Management:

- [ ] Categories/folders for password entries
- [ ] Search functionality
- [ ] Import/export capabilities
- [ ] Password history tracking

#### Architecture:

- [ ] Database migrations
- [ ] Configuration management improvements
- [ ] Transaction support
- [ ] State management refactoring


### Low Priority 🟢

#### Advanced Features:
- [ ] Browser integration
- [ ] Secure password sharing
- [ ] File attachment support
- [ ] Emergency access protocol

#### Quality of Life:
- [ ] Interactive mode
- [ ] Configuration profiles
- [ ] Bulk operations support
- [ ] Plugin system


## Current Improvements in Progress

### Code Quality

- [ ] Enhancing error handling with descriptive messages
- [ ] Expanding test coverage
- [ ] Implementing comprehensive logging
- [ ] Code refactoring for maintainability

### Performance

- [ ] Query optimization
- [ ] Resource usage improvements
- [ ] Caching implementation

### Documentation

- [ ] API documentation
- [ ] Usage examples
- [ ] Development guidelines
- [ ] API documentation


## Tech Stack

1. Core: Rust
2. Database: Sqlite3
3. Encryption: AES-256
4. Configuration: Toml



## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
