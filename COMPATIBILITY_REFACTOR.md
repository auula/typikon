# Typikon Compatibility Refactor

This document describes the compatibility and reliability refactor implemented on the
`refactor/compatibility-hardening` branch.

## Goals

The refactor keeps existing `settings.yml` and `root.yml` projects working while improving:

- filesystem safety;
- Windows, macOS, and Linux path compatibility;
- deterministic output;
- deployment under a URL subpath;
- CLI exit codes and error reporting;
- watch and HTTP server lifecycle handling;
- search index safety and size;
- automated test reliability.

The minimum supported Rust version is 1.79.

## Configuration Compatibility

Existing settings remain valid. The only new field is optional:

```yaml
base_url: ""
```

Use an empty value when the generated site is hosted at a domain root:

```text
https://docs.example.com/
```

Use a path prefix for a project site:

```yaml
base_url: /typikon-book
```

This generates URLs such as:

```text
/typikon-book/index.html
/typikon-book/assets/typikon.css
/typikon-book/english/quickstart.html
/typikon-book/data.json
```

Leading and trailing slashes are normalized. Absolute URLs, query strings, fragments,
backslashes, and parent-directory components are rejected for `base_url`.

## Filesystem Safety

All configured project paths are validated before a build starts.

The following values are rejected:

- absolute input, output, theme, chapter, or subchapter paths;
- paths containing `..`;
- an output path that points to the project root;
- an output path that overlaps the input or theme directory;
- symbolic links in any existing output-path component;
- Markdown symlinks that resolve outside the configured input directory;
- duplicate chapter routes;
- files without a `.md` extension.

These rules prevent accidental deletion of source files and prevent configured content paths
from reading or writing outside the project.

## Atomic Builds

The builder no longer removes the published output before rendering.

The new sequence is:

1. Validate the complete project configuration and every Markdown source.
2. Generate assets, pages, and `data.json` in a unique sibling directory.
3. Rename the existing output to a temporary backup.
4. Rename the completed build to the configured output path.
5. Remove the backup only after the new output is published.

If rendering fails, the temporary build is removed and the previous output remains available.
If publishing fails, Typikon attempts to restore the previous output.

Calling `generate_books` repeatedly on the same `Builder` resets document IDs and search data,
so repeated builds do not accumulate duplicate entries.

## Route Generation

Route generation uses `Path` components instead of string concatenation. URL separators are
always `/`, including when Typikon runs on Windows.

For example:

```yaml
chapters:
  - title: Guides
    path: Guides.md
    sub_chapters:
      - title: Quick Start
        path: Setup/Quick Start.md
```

The generated files are:

```text
<output>/guides/index.html
<output>/guides/setup/quick-start.html
```

Parent directories for nested subchapters are created automatically.

## CLI Behavior

The package version, banner version, and `typikon --version` output all come from
`Cargo.toml` and currently report `0.1.5`.

Commands now return errors to `main`. A failed command exits with a non-zero status, allowing
shell scripts and CI jobs to detect failures correctly.

Supported command aliases include:

```text
typikon help
typikon --help
typikon -h
typikon version
typikon --version
typikon -V
```

Commands reject unexpected positional arguments instead of silently ignoring them.

## Serve And Watch

`serve` validates the project and requires a previously built output directory. It uses a
fallible bind operation and reports port conflicts as normal errors instead of panicking.
`Ctrl-C` triggers graceful HTTP server shutdown.

`watch` performs an initial build before binding the server. It watches:

- the Markdown input directory;
- the active theme directory;
- `settings.yml`;
- `root.yml`.

One debounced event batch triggers one rebuild, even if multiple paths changed. When
`settings.yml` changes successfully, watch mode reloads path settings and restarts the server
when the output directory or port changed. Invalid intermediate settings are logged while the
watch process continues using the last valid configuration.

## Initialization Safety

`typikon init` now requires an empty working directory. This prevents downloaded template files
from replacing existing project data.

The download process:

- has a 60-second HTTP timeout;
- checks HTTP error status codes;
- writes to `repo.zip.part` until the response is complete;
- rejects unsafe ZIP entry paths;
- checks all destination conflicts before moving any extracted files;
- removes archive artifacts after success or failure.

## Search And Template Safety

The search index stores plain Markdown text instead of rendered HTML. This reduces index size
and prevents HTML tags from influencing search matches.

Search result titles are created with DOM nodes and assigned through `textContent`. They are no
longer concatenated into `innerHTML`, which prevents a chapter title from injecting markup.

The template now renders the configured author in metadata and the footer. Internal resource
URLs respect `base_url`, while external HTTP resources remain unchanged. JavaScript receives
`base_url` through JSON encoding.

## Tests

The previous live-server test called a permanent process and could never complete. It has been
replaced with bounded tests that use isolated temporary projects.

The suite covers:

- safe and unsafe path validation;
- output-parent symbolic links on Unix;
- nested chapter generation;
- deterministic search document ordering;
- repeated builds with one `Builder`;
- preservation of the previous output after a failed rebuild;
- duplicate output routes;
- legacy settings without `base_url`;
- base URL normalization;
- Markdown HTML and plain-text conversion;
- CLI parsing, version output, and failure exit codes;
- debounced event batch behavior;
- initialization move conflicts.

Run the complete local validation with:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets
cargo audit --no-fetch
cargo run --locked -- build
```

CI runs formatting, Clippy, and tests on Rust 1.79 and stable Rust across Ubuntu, macOS, and
Windows. Coverage and dependency auditing run as separate jobs.

## Migration Notes

Most existing projects require no changes. Review these behavior changes before upgrading:

1. All configured Markdown files must exist when the builder is created.
2. All project paths must remain inside the project directory.
3. The output path cannot be a symlink or overlap source directories.
4. `typikon init` requires an empty directory.
5. CLI command handler functions now return `TypikonResult<()>`.
6. Failed CLI commands now return a non-zero exit code.
7. Search index `content` contains plain text instead of HTML.

For GitHub Pages project sites, set `base_url` to the repository path before building.

## Code Map

- `src/book/settings.rs`: configuration parsing and resolved directory validation.
- `src/book/builder.rs`: route validation, rendering, search indexing, and atomic publishing.
- `src/utils/file.rs`: portable paths, URLs, archive handling, and safe file operations.
- `src/cli/commands.rs`: command errors, server startup, watch reconfiguration, and init flow.
- `src/cli/args.rs`: command parsing, help, banner, and package version output.
- `src/html/markdown.rs`: HTML and plain-text Markdown rendering.
- `src/html/template.rs`: template view model and base URL resource handling.
- `theme/typikon-theme/index.html`: URL-prefixed assets and safe search-result DOM updates.
- `tests/`: isolated compatibility, CLI, rendering, and filesystem tests.
