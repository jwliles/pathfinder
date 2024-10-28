# **README**

# pathmaster

## **Table of Contents**

- [**README**](#readme)
- [pathmaster](#pathmaster)
  - [**Table of Contents**](#table-of-contents)
  - [**Introduction**](#introduction)
- [pathmaster](#pathmaster-1)
  - [New in Version 0.2.3](#new-in-version-023)
  - [Features](#features)
    - [Backup Management](#backup-management)
  - [New in Version 0.2.2](#new-in-version-022)
  - [Features](#features-1)
    - [Core Features (Updated in 0.2.2)](#core-features-updated-in-022)
    - [Path Management](#path-management)
    - [Safety Features](#safety-features)
  - [Usage](#usage)
    - [Backup Mode Configuration](#backup-mode-configuration)
    - [Checking PATH Sources](#checking-path-sources)
    - [Flushing Invalid Paths](#flushing-invalid-paths)
  - [Configuration Files](#configuration-files)
  - [Documentation](#documentation)
  - [Technical Details](#technical-details)
  - [Upgrading](#upgrading)
  - [Known Issues](#known-issues)
  - [Coming in Future Releases](#coming-in-future-releases)
  - [**Features**](#features-2)
  - [**Installation**](#installation)
    - [**Prerequisites**](#prerequisites)
    - [**Building from Source**](#building-from-source)
  - [**Usage**](#usage-1)
    - [**Command Overview**](#command-overview)
    - [**Commands**](#commands)
      - [**add**](#add)
      - [**remove**](#remove)
      - [**list**](#list)
      - [**check**](#check)
      - [**flush**](#flush)
      - [**history**](#history)
      - [**restore**](#restore)
    - [**Examples**](#examples)
  - [**Configuration**](#configuration)
  - [**Backup Management**](#backup-management-1)
    - [**Backup Modes**](#backup-modes)
    - [**Backup Storage**](#backup-storage)
  - [**Contributing**](#contributing)
  - [**License**](#license)

## **Introduction**

**pathmaster** is a powerful command-line tool written in Rust for managing your system's `PATH` environment variable. It simplifies the process of adding and removing directories from your `PATH`, ensures backups are created automatically, and provides tools to restore previous configurations.

Managing the `PATH` variable is crucial for system performance and command execution. `pathmaster` provides a safe and efficient way to handle `PATH` modifications, with features designed to prevent errors and maintain system stability.

# pathmaster

## New in Version 0.2.3

- Configurable backup modes for selective backup creation
- Interactive mode switching functionality
- Enhanced shell configuration handling
- Improved backup system flexibility
- Better documentation and user feedback

## Features

### Backup Management

- Flexible backup modes:
  - PATH-only backups
  - Shell-only backups
  - Combined backups (default)
  - Easy mode switching
- Automatic backup creation
- Comprehensive backup history
- Safe configuration preservation

## New in Version 0.2.2

- Enhanced PATH validation and scanning capabilities
- Improved detection of PATH modifications across system
- Accurate identification of PATH entry sources
- Better handling of different shell configurations
- Fixed issues with PATH entry detection and validation

## Features

### Core Features (Updated in 0.2.2)

- **Enhanced PATH Scanning**: More accurate detection of PATH modifications
- **Source Identification**: Identifies whether PATH modifications require sudo
- **Improved Validation**: Better handling of different PATH modification formats
- **Shell Support**: Enhanced detection of shell-specific configurations
- **Framework Integration**: Better support for shell framework configurations
- **Safety Features**: Automatic backups, configuration preservation, validation

### Path Management

- Add/remove directories from PATH
- List current PATH entries
- Validate PATH entries
- Automatic backups
- Configuration preservation
- Detailed feedback

### Safety Features

- Automatic backups before modifications
- Shell configuration preservation
- Detailed user feedback
- Recovery options

## Usage

### Backup Mode Configuration

```bash
# Only back up PATH when adding a directory
pathmaster --backup-mode path add ~/bin

# Only back up shell config when flushing invalid paths
pathmaster --backup-mode shell flush

# Reset to backing up both
pathmaster --backup-mode default

# Toggle between backup modes
pathmaster --backup-mode switch
```

### Checking PATH Sources

```bash
pathmaster check
```

Now provides enhanced output showing:

- Invalid directories in PATH
- Source of each PATH modification
- Whether sudo is required for changes
- Shell-specific configuration details

### Flushing Invalid Paths

```bash
pathmaster flush
```

Improved in v0.2.2 with:

- More accurate invalid path detection
- Better feedback for removed paths
- Enhanced shell configuration handling
- Improved backup creation

## Configuration Files

pathmaster now better handles various configuration files:

- Shell-specific files (.bashrc, .zshrc)
- System-wide configurations (/etc/profile, etc.)
- Shell framework configurations
- Distribution-specific locations

## Documentation

Please see the man page (`man pathmaster`) for detailed information about all commands and features.

## Technical Details

For v0.2.2, significant improvements were made to:

- PATH modification detection
- Shell configuration handling
- System vs user file differentiation
- Framework integration
- Error handling and reporting

## Upgrading

When upgrading to v0.2.2:

1. Backup your current configuration
2. Update using your package manager or cargo
3. Review any system-wide PATH modifications
4. Check shell framework compatibility

## Known Issues

None in current release.

## Coming in Future Releases

- Backup mode configuration (v0.2.3)
- Additional shell framework support
- Enhanced configuration options
- Performance optimizations

## **Features**

- **Effortless Management**: Easily add or remove directories from your `PATH`.
- **Automatic Backups**: Creates time-stamped backups of your `PATH` before any changes.
- **Restoration**: Restore your `PATH` from any previous backup.
- **Listing**: View all current entries in your `PATH`.
- **Safe Modifications**: Validates directories before adding them to prevent errors.
- **Persistent Changes**: Updates your shell configuration to make changes permanent.
- **Enhanced Path Validation**: Robust detection and removal of invalid PATH entries
- **Shell Configuration Safety**: Automatic backup of shell configuration files before modifications
- **Detailed Feedback**: Clear reporting of all PATH modifications and their outcomes
- **Session and Permanent Changes**: Updates both current session and shell configuration files

## **Installation**

### **Prerequisites**

- **Rust Toolchain**: Ensure you have Rust installed. You can install Rust using [rustup](https://www.rust-lang.org/tools/install):

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### **Building from Source**

1. **Clone the Repository**

   ```bash
   git clone https://github.com/jwliles/pathmaster.git
   cd pathmaster
   ```

2. **Build the Project**

   ```bash
   cargo build --release
   ```

   This command compiles the project in release mode, producing an optimized binary.

3. **Install the Binary**

   Optionally, you can install the binary system-wide:

   ```bash
   sudo cp target/release/pathmaster /usr/local/bin/
   ```

   Or add it to your `PATH`:

   ```bash
   export PATH="$PATH:$(pwd)/target/release"
   ```

## **Usage**

### **Command Overview**

```bash
pathmaster [COMMAND] [OPTIONS]
```

### **Commands**

#### **add**

Add a directory to your `PATH`.

**Usage:**

```bash
pathmaster add <directory>
```

**Options:**

- `<directory>`: The directory path to add to your `PATH`.

#### **remove**

Remove a directory from your `PATH`.

**Usage:**

```bash
pathmaster remove <directory>
```

**Options:**

- `<directory>`: The directory path to remove from your `PATH`.

#### **list**

List all current entries in your `PATH`.

**Usage:**

```bash
pathmaster list
```

#### **check**

Validate current PATH entries and identify invalid or missing directories.

**Usage:**

```bash
pathmaster check
```

**Example Output:**

```bash
Invalid directories in PATH:
  /home/user/.config/emacs/bin
  /home/user/old/scripts
```

#### **flush**

The `flush` command provides a safe way to remove invalid directories from your PATH:

**Usage:**

```bash
pathmaster flush
# or
pathmaster -f
```

**Process:**

1. Creates a backup of current PATH
2. Creates a backup of shell configuration file
3. Identifies invalid directory entries
4. Removes invalid entries from PATH
5. Updates shell configuration for persistence
6. Provides detailed feedback about changes

**Safety Features:**

- Automatic PATH backup creation
- Shell configuration file backup
- Detailed removal reporting
- Recovery options via backup system
- Session-only fallback if configuration update fails

**Example Output:**

```bash
Created backup of shell config at: /home/user/.bashrc.bak
Removing invalid path: /home/user/.config/emacs/bin
Removing invalid path: /home/user/old/scripts
Successfully removed 2 invalid path(s) and updated shell configuration.
```

#### **history**

Show the backup history of your `PATH`.

**Usage:**

```bash
pathmaster history
```

#### **restore**

Restore your `PATH` from a previous backup.

**Usage:**

```bash
pathmaster restore [--timestamp <timestamp>]
```

**Options:**

- `--timestamp <timestamp>`: (Optional) The timestamp of the backup to restore. If not provided, the most recent backup is used.

### **Examples**

- **Add a Directory to PATH**

  ```bash
  pathmaster add ~/my/custom/bin
  ```

- **Remove a Directory from PATH**

  ```bash
  pathmaster remove ~/my/old/bin
  ```

- **List PATH Entries**

  ```bash
  pathmaster list
  ```

  **Sample Output:**

  ```
  Current PATH entries:
  - /usr/local/bin
  - /usr/bin
  - /bin
  - /usr/local/sbin
  - /usr/sbin
  - /sbin
  - ~/my/custom/bin
  ```

- **Show Backup History**

  ```bash
  pathmaster history
  ```

  **Sample Output:**

  ```
  Available backups:
  - backup_20231007_120000.json
  - backup_20231008_090000.json
  ```

- **Restore PATH from a Specific Backup**

  ```bash
  pathmaster restore --timestamp 20231007_120000
  ```

- **Restore PATH from the Most Recent Backup**

  ```bash
  pathmaster restore
  ```

## **Configuration**

pathmaster now supports configurable backup modes to provide more control over what gets backed up during operations. This can be especially useful when:

- You want to focus on PATH management without shell configuration changes
- You need to manage shell configurations separately
- You want to minimize backup file creation
- You're testing different PATH configurations

`pathmaster` modifies your shell configuration file to make changes to `PATH` persistent across sessions.

- **Supported Shells**: Bash (`.bashrc`), Zsh (`.zshrc`), or a generic `.profile` if the shell is not recognized.
- **Backup Directory**: Backups are stored in `~/.pathmaster_backups`.

**Note**: Always review changes made to your shell configuration files. `pathmaster` adds an export command to update your `PATH`.

## **Backup Management**

pathmaster provides flexible backup management with configurable backup modes:

### **Backup Modes**

Use the `--backup-mode` flag to control what gets backed up:

- `default`: Back up both PATH and shell configurations (default behavior)
- `path`: Back up only PATH entries
- `shell`: Back up only shell configuration
- `switch`: Toggle between PATH-only and shell-only backups

**Examples:**

```bash
# Only back up PATH when adding a directory
pathmaster --backup-mode path add ~/bin

# Only back up shell config when flushing invalid paths
pathmaster --backup-mode shell flush

# Reset to backing up both
pathmaster --backup-mode default

# Toggle between backup modes
pathmaster --backup-mode switch
```

### **Backup Storage**

- **PATH Backups**: Stored as JSON files in `~/.pathmaster_backups`
- **Shell Configuration Backups**: Created as `.bak` files alongside your shell config:
  - Bash: `~/.bashrc.bak`
  - Zsh: `~/.zshrc.bak`
  - Generic: `~/.profile.bak`

## **Contributing**

Contributions are welcome! Please follow these steps:

1. **Fork the Repository**

2. **Create a Feature Branch**

   ```bash
   git checkout -b feature/your-feature
   ```

3. **Commit Your Changes**

   ```bash
   git commit -am 'Add your feature'
   ```

4. **Push to the Branch**

   ```bash
   git push origin feature/your-feature
   ```

5. **Create a Pull Request**

## **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
