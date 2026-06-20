# Implementation Plan for `th` (Terminal Helper)

## Phase 1: Project Setup and CLI Parsing
- [x] Initialize a new Rust binary project (`cargo init`).
- [x] Add dependencies for CLI parsing (e.g., `clap`).
- [x] Add `tokio` for async runtime.
- [x] Implement the basic CLI interface to accept a natural language query as an argument.
- [x] Add basic flag support (e.g., `--help`, `--version`).

## Phase 2: Configuration Management
- [x] Implement a `th --setup` command to guide the user through setting up their API key.
- [x] Save the configuration (e.g., Gemini API Key) securely in the standard user config directory (e.g., `~/.config/th/config.toml`).
- [x] Log the exact path where the configuration was saved so the user knows where it is stored.
- [x] Load the configuration on regular runs.

## Phase 3: LLM Integration (Gemini)
- [x] Choose an HTTP client (e.g., `reqwest`) and add it to dependencies.
- [x] Add serialization/deserialization support (e.g., `serde`, `serde_json`).
- [x] Construct the system prompt to guide the Gemini LLM to *only* output the exact command without markdown formatting or extra text.
- [x] Implement the API call to the Gemini API (using the fast `gemini-1.5-flash` model).
- [x] Handle API errors gracefully and provide helpful error messages to the user.

## Phase 4: System Context Enrichment (UX Improvement)
- [ ] Automatically detect the user's Operating System (macOS, Linux, Windows).
- [ ] Automatically detect the user's current shell (zsh, bash, fish).
- [ ] Inject this system context into the LLM prompt to ensure the returned command is accurate for the user's specific environment.
- [ ] **(Future/Optional)** Explore sending the current directory path and basic file listing (`ls`) as context to the LLM to improve directory-specific queries.

## Phase 5: Output and Interactivity (UX Improvement)
- [ ] Print the suggested command clearly to the terminal.
- [ ] Implement an interactive prompt after displaying the command.
    - Options: `[R]un`, `[C]opy to clipboard`, `[E]dit`, `[A]bort`.
    - **Edit Implementation**: Use an inline prompt (e.g., via the `inquire` or `rustyline` crate) pre-filled with the suggested command, allowing the user to modify it directly in their terminal before running or copying.
- [ ] Implement clipboard support (e.g., using `arboard`) for copying the command.
- [ ] Implement execution support (e.g., `std::process::Command`) to run the command if the user chooses to.

## Phase 6: Polish and Release
- [ ] Write tests for core logic.
- [ ] Setup a basic CI/CD pipeline (GitHub Actions).
- [ ] Publish to crates.io or provide installation instructions.
