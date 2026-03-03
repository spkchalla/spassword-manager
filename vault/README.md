# Vault (v0.1)

A CLI-based password vault built in Rust.

This project is designed primarily as a learning-oriented systems exercise.
The goal of v0.1 is to build a clean, structured, deterministic binary vault format without encryption.
Future versions may introduce encryption, stronger security guarantees, and expanded functionality.

---

# Project Philosophy (v0.1 Scope)

Version 0.1 focuses on:

* Clean module separation
* Deterministic binary file format
* Explicit serialization/deserialization
* Structured error handling
* CLI-based interaction
* Git-driven incremental development

Version 0.1 does NOT include:

* Encryption
* OS-specific integrations
* GUI
* Advanced memory protections

---

# Repository Structure

```
vault/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── commands/
│   │   ├── init.rs
│   │   ├── add.rs
│   │   ├── list.rs
│   │   ├── get.rs
│   │   └── delete.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── vault_format.rs
│   │   └── errors.rs
│   └── storage/
│       ├── mod.rs
│       └── file_io.rs
└── README.md
```

---

# File and Module Responsibilities

## Cargo.toml

* Defines project metadata
* Manages dependencies
* Configures build settings

This project uses stable Rust only.

---

## src/main.rs

Entry point of the application.

Responsibilities:

* Initializes CLI parsing
* Routes commands to appropriate modules
* Handles top-level error reporting

This file should remain thin.
It should not contain business logic.

---

## src/cli.rs

Responsible for:

* Parsing command-line arguments
* Mapping CLI inputs to internal command handlers

This layer only interprets user intent.
It does not manipulate vault data directly.

---

## src/commands/

Each file here represents one CLI command.

### init.rs

Creates a new vault file with a valid header and zero entries.

### add.rs

Adds a new entry to the vault.

### list.rs

Lists metadata of stored entries (e.g., IDs, titles).

### get.rs

Retrieves and displays a specific entry.

### delete.rs

Removes an entry from the vault and updates the file.

Commands should:

* Validate input
* Call core logic
* Return structured errors
* Avoid direct file format manipulation

---

## src/core/

This is the heart of the application.

No CLI logic.
No direct filesystem assumptions.

Pure domain logic only.

### mod.rs

Exports core modules for external use.

### models.rs

Defines core data structures:

* `Entry`
* `Vault` (optional abstraction)

These structs represent in-memory representations of vault data.

They do NOT contain file IO logic.

---

### vault_format.rs

Responsible for:

* Defining binary layout
* Serializing vault headers
* Serializing entries
* Deserializing entries
* Handling deterministic byte encoding

Rules enforced here:

* Fixed-width integer types
* Little-endian encoding
* UTF-8 string encoding
* Explicit length prefixes

This file defines the on-disk format contract.

---

### errors.rs

Defines a custom error enum for the application.

Responsibilities:

* Domain-level error modeling
* IO error wrapping
* Format validation errors
* Entry lookup failures

All core functions should return:

```
Result<T, VaultError>
```

No unwrap or panic in core logic.

---

## src/storage/

This module handles interaction with the filesystem.

### mod.rs

Exports storage functionality.

### file_io.rs

Responsible for:

* Opening vault files
* Reading raw bytes
* Writing serialized data to disk
* Managing file creation and overwriting

This module does NOT define format logic.
It only moves bytes between disk and memory.

---

# Vault File Format (v0.1)

Header:

```
[4 bytes]  MAGIC = "SPKM"
[2 bytes]  VERSION = 1 (little-endian)
[4 bytes]  ENTRY_COUNT (u32 LE)
```

Each Entry:

```
[4 bytes]  id (u32 LE)
[2 bytes]  title_length (u16 LE)
[? bytes]  title (UTF-8)
[2 bytes]  username_length
[? bytes]  username
[2 bytes]  password_length
[? bytes]  password
```

Design Rules:

* All integers are little-endian.
* All strings are UTF-8.
* No raw struct dumping.
* No platform-dependent types (no usize in file format).

---

# Development Workflow

* Small, focused commits
* No large feature jumps
* Test serialization/deserialization round trips
* Avoid unwrap in core modules
* Keep CLI thin

Each milestone should be committed independently.

---

# First Five Development Tasks

After creating the repository, follow this order:

1. Create folder structure and make the project compile.
2. Implement `Entry` and optional `Vault` structs in `models.rs`.
3. Define `VaultError` enum and integrate `Result` return types.
4. Implement header serialization and deserialization.
5. Implement entry serialization and deserialization with round-trip testing.

Only after these steps should CLI commands be wired.

---

# Future Directions (Post v0.1)

* Add encryption layer (Argon2 + AES-GCM or ChaCha20-Poly1305)
* Introduce master password
* Add integrity validation
* Write formal file format specification
* Expand test coverage
* Consider cross-compilation targets

---

# Goal of This Project

This project serves as:

* A Rust systems-learning exercise
* A binary format design practice
* A foundation for future secure storage systems
* A stepping stone toward deeper systems and security engineering

The focus is correctness, structure, and clarity.

Security hardening comes in later versions.

