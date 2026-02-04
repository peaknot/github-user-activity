https://roadmap.sh/projects/github-user-activity

# GitHub Activity CLI 

A CLI tool  that fetches and displays recent activity for any GitHub user.

## Features

* **Fetch Activity:** Retrieves the latest public events for a specified user.
* **Filtering:** Filter events by type using subcommands.

## Installation & Usage

Ensure you have Rust and Cargo installed. Clone the repository and navigate to the project folder.

### 1. Basic Usage
Fetch all recent events for a user:

```bash
cargo run -- <username>
# Example:
cargo run -- torvalds
```
### 2. Filtering Events

```bash
cargo run -- <username> push-event
# or use the alias:
cargo run -- <username> pe
```
```bash
cargo run -- <username> issues-event
# or use the alias:
cargo run -- <username> ie
```
```bash
cargo run -- <username> watch-event
# or use the alias:
cargo run -- <username> we
