# th (Terminal Helper)

[![Release](https://github.com/tnmyk/th/actions/workflows/release.yml/badge.svg)](https://github.com/tnmyk/th/actions/workflows/release.yml)

`th` is a blazing fast, lightweight CLI tool written in Rust that uses AI to help you find and run the right terminal commands without leaving your shell. 

Instead of Googling or digging through `man` pages, simply ask `th` what you want to do. It automatically detects your OS and shell to provide an accurate, ready-to-run command.

## Features
- **Speed**: Powered by Google's `gemini-3.1-flash-lite` model for near-instantaneous responses.
- **Context-Aware**: Automatically injects your OS (macOS/Linux/Windows) and current shell (zsh/bash/fish) into the prompt for perfect accuracy.
- **Interactive Execution**: Drops you into a live-edit prompt. Hit `Enter` to run the command immediately, tweak it inline, or hit `Ctrl+C` to safely abort.
- **History Integration**: Automatically saves executed commands to your shell history (`.zsh_history` / `.bash_history`) so they are accessible via the UP arrow.

## Installation

The easiest way to install `th` on macOS or Linux is using the automatic install script, which automatically downloads the pre-compiled binary for your system:

```sh
curl -fsSL https://th.tanmay.sh/install.sh | bash
```

### From Source (Requires Rust)
If you prefer to compile from source, ensure you have Rust and Cargo installed, then run:

```sh
cargo install --git https://github.com/tnmyk/th.git
```

## Setup

Before using `th`, you need to configure it with a Google Gemini API key. You can get one for free from Google AI Studio.

Run the interactive setup wizard:
```sh
th --setup
```

### Shell Configuration (Optional)
For commands executed by `th` to appear *instantly* when you press the UP arrow in your currently open terminal window, your shell must be configured to share history.
- **Zsh**: Ensure you have `setopt INC_APPEND_HISTORY` or `setopt SHARE_HISTORY` in your `~/.zshrc` (enabled by default in Oh-My-Zsh).
- **Bash**: Ensure you have `shopt -s histappend` and `PROMPT_COMMAND="history -a; history -n; $PROMPT_COMMAND"` in your `~/.bashrc`.

## Usage

Just pass your natural language query directly to `th`:

```sh
th "find all text files recursively and delete them"
```

The tool will display a spinner, fetch the command, and present it to you in an edit prompt:
```
Command: find . -name "*.txt" -type f -delete
```
Press **Enter** to run, or modify it before executing.

## How it works
On running `th` with a natural language description, it makes a secure API call to the Gemini API. It enforces strict constraints so the AI only returns the raw command string, which is then passed back to your terminal for review and execution.