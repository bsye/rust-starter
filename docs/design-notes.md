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
`src/components/` and themes them through `assets/dx-components-theme.css`. That is the stock-component route for
Dioxus.

- **Left as generated, with one patch.** `src/components/` is `dx` output, so `dx components update` can refresh it.
  Our own shared UI lives in `src/ui/`. `dx components add` rewrites the `dioxus-primitives` git line, so the commit pin
  lives in `Cargo.lock`. One patch: the generated `Textarea` rendered `{children}` inside `<textarea>`; empty children
  server-render as `<!--placeholder-->`, which a textarea shows as text. We removed `children` from our copy (upstream
  main still has it). Re-apply this after any `dx components update`.
- **Library crate.** Unused variants in the generated components (`ButtonVariant::Ghost`, ...) failed clippy's dead-code
  check. Moving the module tree into `src/lib.rs` with `pub mod components` makes them public API instead of dead code,
  with no lint suppressions. Our own modules stay private, so dead code there is still caught.
- **Tailwind never overrides component CSS.** Tailwind v4 puts utilities in `@layer utilities`, and the components' CSS
  is unlayered, so the component CSS always wins. To change a property a component sets, wrap it. Properties the
  component leaves open (a skeleton's size) can take Tailwind classes directly.
- **Theme tokens into Tailwind.** `tailwind.css` maps the components' theme variables to Tailwind colours
  (`bg-background`, `text-muted`, `border-border`) with `@theme inline`, so pages use the same tokens as the components
  and follow the system light or dark setting.
- **Button-styled links.** `ui::ButtonLink` applies the button's own `style.css` through `css_module` to a router
  `Link`, so navigation stays a real link (new tab, middle click) and looks like the stock button.
- **Active nav.** `Route::in_notes()` marks the whole notes section (list, any page, detail, new) with `data-active`.
  The router's own `aria-current` only matches the exact URL, so it went dark on `/notes?page=2` and `/notes/7`.
- **Icons.** [dioxus-icons](https://github.com/dioxuslabs/dioxus-icons) (Lucide, the icon family shadcn uses). The
  generated pagination already depends on it, so it replaced `dioxus-free-icons` to keep one icon set.

## 404 status

The catch-all route renders the not-found page and calls `FullstackContext::commit_http_status(NOT_FOUND)`, so server
rendering answers with a real 404 instead of a 200 page that says "not found".

## Tailwind binary baked into the image

On a fresh tools volume, `dx serve` downloads Tailwind and then fails to start its watcher, so the first run had no CSS
until a restart. The dev image now downloads the exact binary `dx` 0.7.10 expects (`TAILWIND_VERSION=4.1.5`) to its
tools folder, and the named volume is seeded from the image. Bump `TAILWIND_VERSION` with `DIOXUS_CLI_VERSION`.

## Tests without a Dioxus test runner

Dioxus 0.7 has no integrated test support (listed as planned in the release notes). Logic uses `cargo test`, database
code uses `#[sqlx::test]` (a fresh database per test, wrapped in a SeaORM connection), and components render to HTML
with `dioxus-ssr` 0.7.10.

## Production image

Based on the [Dioxus deploy tutorial](https://dioxuslabs.com/learn/0.7/tutorial/deploy), with three changes:

- **Cache mounts instead of cargo-chef.** The tutorial cooks dependencies with `cargo chef cook --release`. `dx bundle`
  builds with its own profiles (`wasm-release`, `server-release`) and extra compiler flags, so the cooked artifacts never
  matched: the first image build spent 218 s cooking, then `dx bundle` recompiled all 334 browser and 523 server crates
  anyway. BuildKit cache mounts on the cargo registry and `target/` reuse whatever dx compiled last time, with no coupling
  to dx internals. The trade-off: cache mounts live on the machine that builds, so a CI runner needs its own cache step.
- **Slim, non-root runtime.** The tutorial runs from the full Rust image. Ours is `debian:trixie-slim` (same glibc as the
  `rust:*-trixie` builder) with `ca-certificates` and `curl` for the healthcheck, running as an unprivileged `app` user.
- **Clean asset build.** `.dockerignore` drops `assets/tailwind.css`, so the image always compiles Tailwind from source
  the way a fresh clone does.
- **Pre-compressed assets.** `[web] pre_compress = true` in `Dioxus.toml` makes `dx bundle` write brotli copies, and the
  Dioxus server sends them to browsers that accept `br`: the wasm download fell from 3,209,747 to 645,592 bytes.

| Build | Time |
|---|---|
| cargo-chef, first build | 218 s cooking + 241 s `dx bundle` (cook unused) |
| Cache mounts, cold | 249 s |
| Cache mounts, after a source change | 31 s |

## Server functions are thin

`api/` holds what both sides share: the `Note` and `NotePage` structs, `NoteInput` with its validation, and the server
functions. Each server function only pulls the database out of the request (`Extension<DatabaseConnection>`, added as a
router layer) and calls a plain `async fn` in `server/notes.rs`. Tests call those functions directly with a fresh
database, without HTTP.

## One error type for every call

`ApiError` (`Invalid(FieldErrors)`, `NotFound`, `Server`) is serialisable, so the browser gets the same enum the server
returned, with the status code from `AsStatusCode` (422, 404, 500). Database errors are logged on the server and reach
the browser only as "Something went wrong", never with SQL details.

## Validation runs on both sides

`NoteInput::validated` is shared code. The form runs it before sending, for instant field errors, and the server runs it
again because the server is the authority (`POST /api/notes` with a blank title returns 422).

## Migrations take a lock

SeaORM's migrator takes no lock, so two app copies starting together could both migrate. `db::migrate` runs every
pending migration inside one transaction holding `pg_advisory_xact_lock`. The test `concurrent_migrations_both_succeed`
starts two migrations at once: it failed 3 of 3 runs with the lock removed and passed 3 of 3 with it.

## Pagination by page number

Offset pagination with a total count, because the UI shows "Page 3 of 500" and the stock pagination component is page
based. With 10,000 notes it is well inside budget (below). Keyset pagination would scale past millions of rows but
cannot jump to a page number; revisit if a list grows that large.

Measured on 2026-10-28, debug build, OrbStack on an arm64 Mac, 11 requests per URL:

| URL | Median | Max |
|---|---|---|
| `/notes` (server rendered) | 14.0 ms | 23.9 ms |
| `/notes?page=250` | 13.3 ms | 17.8 ms |
| `/notes?page=500` (last) | 16.6 ms | 23.4 ms |
| `/api/notes?page=1` | 5.3 ms | 9.0 ms |
| `/api/notes?page=500` | 5.3 ms | 6.6 ms |

## Dates are shown in UTC

Server rendering cannot know the visitor's time zone, and formatting differently on each side breaks hydration, so dates
render as `28 Oct 2026` in UTC on both sides (`ui::format::date`).

## Accounts and sessions

| Concern | Decision |
|---|---|
| Passwords | Argon2id with the crate defaults, hashed on a blocking thread. 12 to 128 characters |
| Unknown emails | Sign-in runs a hash check against a decoy hash, so wrong-password and unknown-email answers take the same time and say the same thing. Measured: 334 ms vs 324 ms median (debug build) |
| Sessions | Server-side rows in Postgres (`axum_session`, table `sessions`); the cookie only holds a random id. HttpOnly, SameSite=Lax, Secure whenever `PUBLIC_URL` is https |
| Session fixation | `start_session` renews the session id before logging the user in |
| Sign-out | Removes the user from the session and renews it, so an old cookie gets 401 afterwards (checked with curl) |
| Duplicate emails | Emails are trimmed and lowercased before saving; the unique index turns a duplicate into a field error |
| CSRF | `server/origin.rs` rejects any write (anything but GET, HEAD, OPTIONS) whose `Origin` is not `PUBLIC_URL`, on top of SameSite=Lax. A request with no `Origin` is rejected too: security checks fail closed |

Not built yet: sign-in rate limiting, password reset, email verification, "back to the page you wanted" after sign-in.

## One choke point for note access

Every note query starts from `scoped(actor)`, which filters by owner and hides soft-deleted rows. Note functions take an
`Actor`, which only `server::auth` can create (from the session, or `trusted` for the seed command). Two tests pin it:
`every_note_read_goes_through_scoped` fails if any other code calls `note::Entity::find()`, and
`nobody_sees_or_changes_another_users_notes` checks that list, get, update and delete all answer 404 for someone else's
note.

## Audit trail and soft delete

Every create, update and delete writes an `audit_events` row (actor, action, entity, id, time) in the same transaction as
the change. Deleting sets `deleted_at` instead of removing the row, so no data is lost; `scoped()` hides those rows.

## Signed-in user without suspending

The layout reads the signed-in user with `use_server_cached(request_user)`: on the server it takes the user the session
layer already loaded for this request, and the value travels to the browser inside the page. An earlier version fetched
it with `use_server_future`, which suspended the whole layout and showed the problem below on every page.

## Component stylesheets are linked from the root

`css_module` adds a component's `<link>` from a `static OnceLock` the first time a class is used. In the browser that is
once per page load, which is fine. On the server it is once per process, so after the first request server-rendered
pages had no component CSS and painted unstyled until the app loaded. `ui::ComponentStyles` links every component
stylesheet on every request, using the same `asset!(..., AssetOptions::css_module())` the macro uses, so the URLs match.
`every_generated_component_stylesheet_is_linked` fails when a newly added component is missing from the list.

## Login redirects are real redirects

`ui::Redirect` answers a server-rendered request with `303 See Other` and a `Location` header, and navigates in the
browser otherwise, so a signed-out visit to `/notes` never renders the notes layout.

## Benchmark fairness

Plan in [BENCHMARK.md](BENCHMARK.md); numbers in [bench/REPORT.md](../bench/REPORT.md).

| Decision | Why |
|---|---|
| Each app written the idiomatic way | It is what you would ship. The biggest difference: nuxt-auth-utils keeps the session sealed in the cookie, so sign-out cannot revoke an old cookie (the parity check shows it) and pages skip a session lookup in the database |
| Nuxt runs Nitro's `node_cluster` preset with 2 workers | Both containers get 2 CPUs; a single Node process would use one. Nitro documents the preset for multi-core use |
| Nuxt builds with `compressPublicAssets`, Rust with `pre_compress` | Both serve pre-compressed static assets, as a production deployment should |
| Lighthouse loads `localhost` on the host network | Chrome only offers brotli to secure origins; over `http://rust:8080` it fetched the 3.2 MB wasm uncompressed, over localhost 646 KB, which is what users get over https |
| Seed data inserted with the same SQL for both | Identical rows, independent of each app's seed tool |
| Load: closed loop for throughput, fixed 50 req/s for latency | Closed-loop latency hides queueing (coordinated omission); oha's `--latency-correction` only applies to a fixed rate. 50 req/s sits below both apps' capacity; a first run at 200 req/s overloaded Nuxt's server rendering (about 110 pages/s), so it measured a queue, not latency |
| Cold image builds use a fresh `buildx` builder with base images pulled first | Empties cache mounts without touching other projects' Docker caches, and keeps download time of base images out of the build time |
| Nuxt dev container reinstalls dependencies only when the lockfile changes | Running `npm install` on every start made restarts 35 s; that measured the Dockerfile, not Nuxt |
| Generated code counted apart | shadcn-vue and dioxus-components output, Drizzle's generated migrations and both theme stylesheets are not hand-written |
| Two warm-ups | 5 s before each load run; one unmeasured check run before timing checks. A cold JIT made Nuxt look half as fast in a first probe |
