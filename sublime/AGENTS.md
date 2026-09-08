# Sublime Text -- Agent Instructions

Follow the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Theme Format

- The port includes native Sublime Text JSON UI themes and color schemes:
  - `Warm Burnout Dark.sublime-theme`
  - `Warm Burnout Light.sublime-theme`
  - `Warm Burnout Dark.sublime-color-scheme`
  - `Warm Burnout Light.sublime-color-scheme`
- The UI themes extend Sublime Text's built-in Default theme and override its color variables. The color schemes configure the editor and syntax highlighting.
- Users can install the files in the `User` package opened through **Preferences: Browse Packages**.
- The `name` field must remain `Warm Burnout Dark` or `Warm Burnout Light`.

## Scope Mapping

- Use TextMate scope selectors. Prefer general scopes first, then more specific selectors for regex, operators, built-ins, and CSS properties.
- **bold** = keywords, storage, markup headings, and HTML tags.
- *italic* = comments, types/classes, decorators, CSS properties, and language variables.
- Normal = functions, strings, constants, operators, variables, and punctuation.
- Do not style broad `meta.*` selectors unless required for a syntax that has no stable semantic scope.

## Color Rules

1. Use exact hex values from the canonical palette for syntax tokens.
2. Background and foreground must match the editor core: `#1a1510` / `#bfbdb6` for dark and `#F5EDE0` / `#3a3630` for light.
3. Cursors use canonical gold: `#f5c56e` dark and `#8a6600` light.
4. Selection, line highlight, and gutter values match the pre-blended values used by Bat.
5. Dark syntax tokens must meet AAA against `#1a1510`. Light syntax tokens must meet AA against `#F5EDE0`.

## Testing

Run:

```sh
cargo test --test sublime
cargo test --test canonical -- sublime
python3 -m json.tool "Warm Burnout Dark.sublime-theme" >/dev/null
python3 -m json.tool "Warm Burnout Light.sublime-theme" >/dev/null
python3 -m json.tool "Warm Burnout Dark.sublime-color-scheme" >/dev/null
python3 -m json.tool "Warm Burnout Light.sublime-color-scheme" >/dev/null
```

For a manual check, copy all four files into the `User` package. Select a UI theme from **Preferences: Select Theme**, then select its matching scheme from **Preferences: Select Color Scheme**. Check TypeScript, HTML/JSX, CSS, Python, Rust, YAML, Markdown, and Diff.
