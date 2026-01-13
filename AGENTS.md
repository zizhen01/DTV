# AGENTS.md

This file guides agentic coding assistants working in this repo.
It summarizes build/lint/test commands and code style conventions.

## Quick facts
- App is a Vue 3 + Vite + TypeScript frontend.
- Desktop shell is Tauri (Rust in `src-tauri`).
- Package manager: `pnpm` (per README).
- TypeScript is strict (`tsconfig.json`).

## Cursor/Copilot rules
- No `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` found.

## Commands (from package.json/README)
### Install
- `pnpm install`

### Frontend dev/build
- `pnpm dev` (Vite dev server only)
- `pnpm build` (typecheck + Vite build)
- `pnpm preview`

### Tauri dev/build (from README)
- `pnpm tauri dev`
- `pnpm tauri build`
- `pnpm tauri build --target aarch64-apple-darwin`

### Rust (src-tauri)
- `cargo build` (run in `src-tauri`)
- `cargo test` (run in `src-tauri`)
- Single test: `cargo test <test_name>` or `cargo test module::test_name`

### Linting
- No ESLint/Prettier configs or lint scripts found.
- TypeScript linting is enforced via `tsconfig` (`strict`, `noUnusedLocals`, etc.).
- Use `pnpm build` or `pnpm dev` to surface TS errors.

### Formatting
- No formatter config found; avoid introducing new formatting tools.
- Follow existing file formatting and spacing.

## Frontend code style (Vue/TS)
### Language & framework
- Vue 3 Single File Components with `<script setup lang="ts">`.
- Composition API with `ref`, `computed`, `reactive`, `onMounted`, etc.
- Prefer `import type` for type-only imports.

### Imports
- Use ES module imports.
- Group imports by origin: Vue/libs, local modules, local components, styles.
- Keep related imports together (see `src/components/player/index.vue`).
- Use relative paths within `src`, avoid deep `../../../` when a local module exists.

### Types
- Use explicit types for props, store state, and external data.
- Prefer `interface` for store state shapes, `type` for unions and aliases.
- Use `as const` where fixed literal arrays are needed.
- Avoid `any` unless interacting with 3rd‑party APIs; document intent with naming.

### Naming
- `camelCase` for variables, functions, and refs.
- `PascalCase` for components and classes.
- Files: components use `PascalCase.vue`, composables `useXxx.ts`.
- Stores follow `useXxxStore` (Pinia).
- Constants use `SCREAMING_SNAKE_CASE`.

### State & stores
- Use Pinia stores in `src/stores`/`src/store`.
- Side effects in store actions, not in state.
- Wrap localStorage access in try/catch where errors are possible.

### Error handling & logging
- Use `try/catch` around async operations and I/O.
- Log with `console.error`/`console.warn` plus context prefix, e.g. `[Player]`.
- Preserve user-facing errors as strings for UI display.

### Vue templates
- Keep template logic readable; prefer computed values for complex conditions.
- Use `v-if`/`v-else-if` for mutually exclusive states.
- Keep class names kebab-case.

### CSS
- CSS lives in component folders or `src/styles`.
- Prefer local component styles where possible.
- Keep spacing consistent; no formatter assumptions.

## Rust/Tauri code style
### Module layout
- Rust sources live in `src-tauri/src`.
- Platform-specific logic under `src-tauri/src/platforms`.
- Tauri commands use `#[tauri::command]` in `src-tauri/src/main.rs`.

### Naming & structure
- Use `snake_case` for functions, modules, and variables.
- Structs/enums use `PascalCase`.
- Prefer explicit type names over `impl Trait` in public APIs unless idiomatic.

### Error handling
- Use `Result<T, String>` for Tauri command errors.
- Map errors with `map_err` and include context in error messages.
- Log errors via `eprintln!` or `println!` with clear prefixes.
- Avoid panics; use `expect` only for truly unrecoverable setup.

### Async
- Use `tokio::spawn` for background tasks.
- Keep command functions `async` when they touch I/O.
- Propagate errors instead of swallowing them.

### External I/O
- Use the shared `reqwest::Client` managed by Tauri (`main.rs`).
- When adding new HTTP calls, reuse the managed client.

## Working conventions
- Keep changes minimal and consistent with current style.
- Do not introduce new dependencies unless required.
- Update both frontend and Rust if changes cross the Tauri boundary.
- Match existing logging strings (English prefixes + localized messages).
- Avoid unrelated refactors in large Vue components.

## Common paths
- Frontend entry: `src/main.ts`
- Routes: `src/router/index.ts`
- Pinia stores: `src/stores/*` and `src/store/*`
- Tauri main: `src-tauri/src/main.rs`
- Tauri proxy: `src-tauri/src/proxy.rs`
- Tauri platforms: `src-tauri/src/platforms/**`

## Suggested single‑test workflows
- Rust: `cargo test <test_name>` in `src-tauri`
- Rust (module): `cargo test platforms::douyin::tests::it_works`
- JS/Vue: no test runner configured; add only if requested.

## Notes for agents
- Treat `src-tauri/target` as build output; do not edit.
- Be careful with platform‑specific behavior (macOS/Windows/Linux).
- For UI changes, validate both light/dark themes.
- Prefer `void` with fire‑and‑forget promises in Vue (`void maybeCheckForUpdates()`).
- When using Tauri `invoke`, ensure command name matches Rust handler.

## How to extend
- If you add new scripts, update this file.
- Keep sections concise and scannable.
- Maintain ~150 lines for readability.

## Localization & strings
- UI strings are primarily Chinese; keep language consistent.
- Prefer reuse of existing message patterns.
- For new errors, include both context prefix and user message.
- Avoid emojis unless already used in UI copy.
