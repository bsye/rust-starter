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
| Markup change | Edit text in a page, time until the browser shows it without a reload ([Playwright](https://playwright.dev), 10 runs) |
| Logic change | Edit a server function, time until the new behaviour is served (10 runs) |
| Checks | Lint, typecheck and tests with warm caches |
| Image build | Cold, and after a one-line change |

### Production

| Metric | How |
|---|---|
| Throughput and latency | [oha](https://github.com/hatoo/oha) 1.16, 10 s warm-up, then 30 s at 50 connections with latency correction: signed-in list (page 1 and 250), detail page, `GET /api/notes`; sign-in on its own |
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
| 2 | Harness | `bench/run.sh` and one script per metric |
| 3 | Report | First full run, `bench/REPORT.md`, findings in `design-notes.md` |

## Acceptance criteria

- GIVEN both apps, WHEN the parity checklist runs, THEN each item passes or the difference is listed in the report.
- GIVEN a clean machine with Docker, WHEN I run `bench/run.sh`, THEN every number in `bench/REPORT.md` is regenerated.
- GIVEN any metric, THEN the report shows its median, its spread and the number of runs.

## Known caveats

- **Sessions differ by design.** nuxt-auth-utils seals the session in the cookie, so a request needs no database lookup;
  the starter keeps sessions in Postgres. See open question 2.
- **Dev loop compares dev modes.** Rust dev builds are unoptimised; Nuxt dev runs Vite. Production numbers use release
  builds on both sides.
- **One machine.** arm64 Mac with OrbStack; your other running stacks add noise, which the repeated runs expose but do
  not remove.
- **Same author for both apps.** The Nuxt app follows the official Nuxt, Drizzle and nuxt-auth-utils docs to limit bias.

## Open questions

1. **Baseline stack.** (a) the common defaults above (recommended unless your team has a standard); (b) your team's
   usual Nuxt stack, if it differs.
2. **Parity level.** (a) each app written the idiomatic way, differences documented (recommended: it is what you would
   ship); (b) force identical architecture, for example Postgres sessions in Nuxt too.
3. **Where.** (a) `bench/` in this repo on the `benchmark` branch (recommended); (b) a separate repo.
4. **Languages.** The Nuxt app and the Playwright and Lighthouse scripts are TypeScript, plus shell for the runner. OK?
5. **"Speed of developing".** The plan measures the feedback loop and code size. How long a human takes to build a
   feature cannot be measured fairly, so it is left out. OK?
