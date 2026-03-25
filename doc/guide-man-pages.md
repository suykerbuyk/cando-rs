# Man Pages

This document describes the man page generation system for the Cando-RS project. Man pages are automatically generated from the Clap command-line argument structures defined in each binary, ensuring they accurately reflect the actual CLI options and help text.

## How Man Pages Are Generated

The man page generation system uses Clap's derive macros and the `clap_mangen` crate to produce man pages directly from the source code:

- **Clap Integration**: Each binary defines an `Args` struct using Clap's derive macros for command-line parsing. The `Args::command()` method (from the `CommandFactory` trait) creates a `clap::Command` representing the full CLI structure, including subcommands, flags, and descriptions.
- **Rendering**: When enabled, `clap_mangen::Man::new(command).render()` converts the `Command` into man page format (troff/groff markup).
- **Feature-Gated**: All generation code is wrapped in `#[cfg(feature = "manpages")]` blocks, keeping builds clean when the feature is disabled.
- **Unified Handling**: Binaries using `CommonArgs` inherit the generation flag automatically. Others have it added directly to their argument structures.

Each binary includes a hidden `--generate-manpage` flag. When invoked with this flag and the `manpages` feature enabled, the binary outputs its man page to stdout and exits.

## Using `--features manpages`

The `--features manpages` flag enables man page generation for the entire workspace:

- **Dependency Activation**: Enables `clap_mangen` as an optional dependency in each binary's `Cargo.toml`.
- **Conditional Code**: Activates `#[cfg(feature = "manpages")]` blocks, which include:
  - Imports for `clap::CommandFactory` and `clap_mangen::Man`.
  - Logic to check for `--generate-manpage` and render the man page.
- **CLI Extension**: Adds the hidden `--generate-manpage` flag to affected binaries' command-line interfaces.
- **No Runtime Impact**: The feature only affects build-time and the hidden flag; core binary functionality remains unchanged.

When the feature is disabled, binaries run normally. Using `--generate-manpage` without the feature produces an error: `"clap_mangen feature not enabled. Build with --features manpages to generate man pages"`.

## Generating Man Pages

Man pages are generated using the `generate-man.sh` script, which automates the process for all binaries. This ensures man pages stay up-to-date with CLI changes.

### Automated Generation with `generate-man.sh`

Run `./scripts/generate-man.sh` from the project root:

- **Discovery**: The script hardcodes a list of binaries: `rust-can-util`, `analyze-hvpc`, `can-log-analyzer`, `count-hvpc-signals`, `dump-messages`, `emp-simulator`, `hvpc-simulator`, `investigate-values`, `monitor-can`.
- **Build and Run**: For each binary, it executes `cargo run --bin <binary> --features manpages -- --generate-manpage > man/<binary>.1`.
- **Output**: Man pages are saved as `.1` files in the `man/` directory (e.g., `man/analyze-hvpc.1`).
- **Usage**: Ideal for bulk generation during development, CI/CD, or releases. The script provides options like `--output-dir` for custom paths and `--list-only` to preview binaries without generating.

This method guarantees consistency and accuracy, as man pages reflect the live Clap structures.

### Manual Generation

For testing or single-binary updates:

- Run: `cargo run --bin <binary> --features manpages -- --generate-manpage > path/to/manpage.1`
- Example: `cargo run --bin analyze-hvpc --features manpages -- --generate-manpage > man/analyze-hvpc.1`
- Redirect stdout to save the man page as a `.1` file.

Use this for quick checks or when working on a specific binary's CLI.

## Installing Man Pages

After generation, use `./scripts/install-man.sh` to install man pages system-wide or for the current user:

- **User Installation**: `./scripts/install-man.sh --user` installs to `~/.local/share/man/` (requires `man` to be configured to check this path).
- **System Installation**: `sudo ./scripts/install-man.sh` installs to `/usr/local/share/man/` (standard system location).
- **What It Does**: Copies `.1` files from the `man/` directory to the appropriate man page directories, compresses them (if needed), and updates the man database. It handles permissions and ensures the pages are discoverable via `man <command>`.

Run `man -l man/<binary>.1` to preview a generated man page before installation.

## Best Practices

- **Regenerate After CLI Changes**: Run `generate-man.sh` whenever Clap structures (args, help text) are modified to keep man pages current.
- **CI/CD Integration**: Include man page generation in build pipelines to ensure documentation stays synchronized.
- **Testing**: Use `cargo build --workspace --features manpages` to verify feature-enabled builds compile cleanly.
- **Troubleshooting**: If generation fails, ensure `clap_mangen` is available and the `--generate-manpage` flag is used correctly.

This system provides self-documenting, accurate man pages derived directly from the source code.