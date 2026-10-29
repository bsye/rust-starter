# Benchmark plan: Rust starter vs Nuxt

Compare this starter with an equivalent Nuxt app on features and on speed, both while developing and in production.
Every number comes from one script and can be regenerated.

## The two apps

Same features (the acceptance criteria in [STARTER.md](STARTER.md)), same Postgres 18, same seed data, same machine,
same container limits. Versions checked on 2026-10-28.

| | Rust starter | Nuxt baseline |
|---|---|---|
| Framework | Dioxus 0.7.10 | [Nuxt](https://nuxt.com) 4.6.3 (Vue, Nitro) |
| Runtime | Native binary | Node 26 LTS |
| Database | SeaORM 2.0 | [Drizzle](https://orm.drizzle.team) 0.45 + postgres.js |
| Auth | axum_session, sessions in Postgres | [nuxt-auth-utils](https://github.com/atinux/nuxt-auth-utils) 0.5, sealed cookie sessions |
| UI | dioxus-components (shadcn style) | [shadcn-vue](https://www.shadcn-vue.com) via shadcn-nuxt 2.9 |
| Styling | Tailwind v4 | Tailwind v4 |
| Tests | cargo test + sqlx::test | Vitest 5 + @nuxt/test-utils 4 |

### Rust variants

Each variant is a branch, cloned fresh for every run, so the report measures committed code.

| Variant | Branch | What changes |
|---|---|---|
| Rust, Dioxus 0.7 | `main` | The starter as built |
| Rust, Dioxus 0.7, mold | `faster-builds` | [mold](https://github.com/rui314/mold) linker, line tables only for our crate, no debug info for dependencies |
| Rust, Dioxus 0.8 alpha, mold, hot-patch | `dioxus-0.8` | The mold settings, Dioxus 0.8.0-alpha.3, `dx serve --hot-patch`, and the crate layout hot-patching needs ([design notes](https://github.com/bsye/rust-starter/blob/dioxus-0.8/docs/design-notes.md#hot-patching) on that branch) |

Features and production cover 0.7 and 0.8 (mold only changes dev builds); the developer loop covers all three.

## Metrics

### Features

| Metric | How |
|---|---|
| Parity | The same acceptance checklist run against both apps, pass or fail per item |
| Code size | [scc](https://github.com/boyter/scc) lines of code per area: UI, API, data, auth, tests, config |
| Dependencies | Direct and total, from the lockfiles |
| Built in vs added | What each needed beyond the framework, and what we had to work around |

### Developer loop

| Metric | How |
|---|---|
| First run | Fresh clone, `docker compose up`, first page served, cold caches |
| Restart | Stack restart with warm caches, first page served |
| Markup change | Edit text in a page, time until the browser shows it ([Playwright](https://playwright.dev), 10 runs, full reloads counted) |
| Browser logic change | Edit the count formatter the notes pager calls, time until the browser shows the new count (3 runs, full reloads counted) |
| Server logic change | Edit the notes query's total, time until `GET /api/notes` returns it (3 runs) |
| Checks | Lint, typecheck and tests with warm caches |
| Image build | Cold, and after a one-line change |

### Production

| Metric | How |
|---|---|
| Throughput and latency | [oha](https://github.com/hatoo/oha) 1.16, 5 s warm-up, then 20 s closed loop at 50 connections for throughput and 20 s at a fixed 50 requests/s with latency correction for latency: signed-in list (page 1 and 250), detail page, `GET /api/notes`; sign-in on its own |
| Cold start | `docker run` until `/health` answers |
| Memory | Idle and peak under load (`docker stats`) |
| Image size | `docker images` |
| Page weight and Web Vitals | [Lighthouse](https://github.com/GoogleChrome/lighthouse) 13, mobile preset, median of 5: score, LCP, TBT, JavaScript transferred |

Both production containers run with the same limits (2 CPUs, 1 GB).

## Method

- One Postgres container; each app gets its own database with the same 10,000 notes in one account.
- Runs are sequential. Each metric runs at least 3 times; the report shows the median and the spread.
- The script writes JSON to `bench/results/<date>/` and renders `bench/REPORT.md` from it. No number is typed by hand.

## Phases

| # | Phase | Delivers |
|---|---|---|
| 1 | Nuxt app | Feature parity with the starter, its tests, Docker dev and prod setup, in `bench/nuxt/` |
| 2 | Harness | `bench/run.mjs` and one script per part |
| 3 | Report | First full run, `bench/REPORT.md`, findings in `design-notes.md` |

## Acceptance criteria

- GIVEN both apps, WHEN the parity checklist runs, THEN each item passes or the difference is listed in the report.
- GIVEN a clean machine with Docker, WHEN I run `node bench/run.mjs`, THEN every number in `bench/REPORT.md` is
  regenerated.
- GIVEN any metric, THEN the report shows its median, its spread and the number of runs.

## Known caveats

- **Sessions differ by design.** nuxt-auth-utils seals the session in the cookie, so a request needs no database lookup;
  the starter keeps sessions in Postgres. See open question 2.
- **Dev loop compares dev modes.** Rust dev builds are unoptimised; Nuxt dev runs Vite. Production numbers use release
  builds on both sides.
- **One machine.** arm64 Mac with OrbStack; your other running stacks add noise, which the repeated runs expose but do
  not remove.
- **Same author for both apps.** The Nuxt app follows the official Nuxt, Drizzle and nuxt-auth-utils docs to limit bias.

## Decisions

All five open questions were answered with the recommended option: common Nuxt defaults, idiomatic apps, `bench/` in
this repo, TypeScript and JavaScript for the Nuxt app and harness, and no attempt to time humans.

## Running it

Needs Docker and Node 20 or newer.

```sh
node bench/run.mjs            # everything, about 2 hours
node bench/run.mjs production # one part: features, production or devloop
```

Results land in `bench/results/<date>/` (not committed) and the report in `bench/REPORT.md`. `BENCH_RUNS` changes the
number of repetitions (default 3).
