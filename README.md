# Learning
A collection of miscellaneous learning materials and notes, mostly on technical topics for the software engineer. Topics are loosely separated by directories in project root. The presence of an `exercises.toml` file within a directory means the information is quizzable - the project cli can be run to perform the exercises for practice.

# CLI
## Prerequisites

1. Install Dependencies / C linker
```zsh
# Debian/Ubuntu
sudo apt install build-essential
```
```zsh
# Fedora/RHEL
sudo dnf install gcc
```
```zsh
# MacOS
xcode-select --install
```

2. Install Rust
```zsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Follow the default installation prompts and then reload your shell
```zsh
source $HOME/.cargo/env
```
3. Clone and build
```zsh
git clone https://github.com/wadeptr/learning.git
```
```zsh
cd learning
```
```zsh
cargo build --release
```

## Usage
The cli scans the working directory for subdirectories that contain `exercises.toml` and makes them available with hints through the interactive cli. 
```rust
cargo run           # interactive topic picker
cargo run -- vim    # skip to vim exercises
cargo run -- technical-interview --sample      # random daily sample of 5
cargo run -- technical-interview --sample 8    # choose a sample size
```

Sample mode draws from every exercise in the selected topic, regardless of
level. When possible, a sample of at least two includes both data structures
and algorithms. It stores a small `.quiz-sample-state.toml` history file in the
topic directory so questions do not repeat until the complete topic has been
reviewed. If fewer unseen questions remain than the requested sample size, the
final sample in that cycle is smaller; the next invocation starts a new cycle.
