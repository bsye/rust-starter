# Design notes

Why the starter is built the way it is. The plan lives in [STARTER.md](STARTER.md).

## Dioxus 0.7, not 0.8

0.8 is in alpha (alpha.3 on 2026-10-16) and its milestone is more than a year past its due date. 0.7.10 is the current
stable release; we upgrade once when 0.8 ships.

## One crate, not a workspace

The [Dioxus project setup guide](https://dioxuslabs.com/learn/0.7/essentials/fullstack/project_setup) says a single
crate with `web` and `server` features is enough for simple setups. One `Cargo.toml` keeps the Cargo surface small
while learning. Revisit when compile times or `cfg` noise hurt.

## SeaORM, not raw sqlx

SQL is not a learning goal, so queries and migrations stay in Rust. SeaORM 2.0.4 runs on sqlx 0.9, the same version
axum_session_sqlx uses, so sessions and data share one connection pool. Its paginator returns the total count with the
page, which every list needs. Migrations live in a `server/migrations` module instead of the usual separate crate, which
keeps the project to one crate. Entities are server only; server functions return plain shared structs, so SeaORM never
compiles to wasm.

## axum_session_auth, not axum-login

| | axum-login 0.18 | axum_session_auth 0.22 |
|---|---|---|
| Session layer | tower-sessions 0.14 (current is 0.15) | axum_session 0.22 (axum 0.8.9) |
| Postgres store | tower-sessions-sqlx-store 0.15, on sqlx 0.8 | axum_session_sqlx 0.12, on sqlx 0.9 |
| Last release | 2025-07-20 | 2026-10-22 |
| Downloads, last 90 days | 536k | 23k |

axum-login would pull a second sqlx version next to the 0.9 SeaORM uses. axum_session_auth matches our versions and is the crate the
[Dioxus auth guide](https://dioxuslabs.com/learn/0.7/essentials/fullstack/authentication) names. The cost is a smaller
user base.

## Postgres 18

The newest major published on Docker Hub (`postgres:19` does not exist yet).

## Everything runs in containers

No Rust toolchain on the host. `target/` and the cargo registry sit in named volumes because bind-mounted build output
is slow on macOS. The tools `dx` downloads on first run (Tailwind, wasm-bindgen, esbuild) live in the `dx-tools` volume
so a recreated container does not fetch them again.

The app is published on host port 8090: 8080 is a common default and was already taken on the first dev machine.

## Checks run clippy twice

The server build and the browser build compile different code (`cfg(feature = "server")`), so `scripts/check.sh` lints
each one. A `disallowed-types` rule in `clippy.toml` rejects `regex::Regex`; it was proved by adding regex to a throwaway
copy and watching clippy fail.

## Tailwind through dx

The [0.7 release notes](https://dioxuslabs.com/blog/release-070/) say `dx` starts a Tailwind watcher when a
`tailwind.css` exists at the root, with no separate CLI. The
[Tailwind guide](https://dioxuslabs.com/learn/0.7/guides/utilities/tailwind) still describes an npm install. Confirmed
in phase 1: `dx serve` installed Tailwind v4.1.5 itself. Its output, `assets/tailwind.css`, is generated and ignored by
git.

## Components through dx components add

[dioxus-components](https://github.com/DioxusLabs/dioxus-components) copies shadcn-style components into
`src/components/` and themes them through `assets/dx-components.css`. That is the stock-component route for Dioxus.

## Tests without a Dioxus test runner

Dioxus 0.7 has no integrated test support (listed as planned in the release notes). Logic uses `cargo test`, database
code uses `#[sqlx::test]` (a fresh database per test, wrapped in a SeaORM connection), and components render to HTML
with `dioxus-ssr` 0.7.10.

## Production image

Follows the [Dioxus deploy tutorial](https://dioxuslabs.com/learn/0.7/tutorial/deploy): cargo-chef for dependency
caching, `dx bundle --web --release`, then copy `target/dx/<app>/release/web/` into a slim runtime with `IP` and `PORT`
set.
