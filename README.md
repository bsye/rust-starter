# Rust starter

A containerised Dioxus fullstack scaffold for web apps: Rust on the server and in the browser, Postgres, accounts, and a
production image.

![Notes list](docs/screenshots/notes.jpg)

## Run it

Needs Docker only.

1. `docker compose up --build`
2. Open [localhost:8090](http://localhost:8090) and create an account. The first build takes a few minutes; later edits
   hot reload.
3. In VS Code, run **Dev Containers: Reopen in Container** to get rust-analyzer inside the `app` container.

Fill an account with 10,000 generated notes:

```sh
docker compose exec app cargo run --bin seed --no-default-features --features server -- you@example.com 10000
```

Checks (format, clippy for server and browser, tests; the database tests need the stack running):

```sh
docker compose exec app ./scripts/check.sh
```

## Production image

```sh
docker build --target prod -t rust-starter .
docker run -p 8080:8080 -e DATABASE_URL=postgres://... -e PUBLIC_URL=https://your.domain rust-starter
```

Measured on 2026-10-28 on an arm64 Mac (OrbStack), release build, 10,000 notes in one account, 21 requests per page:

| What | Result |
|---|---|
| Image size | 147 MB (`debian:trixie-slim`, 24 MB server binary, 3.2 MB of browser assets) |
| Cold start to healthy | 574 ms, including migrations on an empty database |
| Memory | 26 to 36 MiB (two runs, idle and after the load test) |
| Browser download of the app | 646 KB wasm (brotli), 3.2 MB unpacked |
| Notes list, server rendered | 4.1 ms median, 9.3 ms p95 |
| Page 250 of 500 | 9.1 ms median, 14.9 ms p95 |
| `GET /api/notes` | 3.9 ms median, 8.5 ms p95 |
| Sign-in (Argon2id) | 67 ms median |
| Image build | 249 s cold, 31 s after a source change |

Built and run on arm64 only. Most hosts need `docker buildx build --platform linux/amd64 --target prod .`, which has not
been run here.

## Tour

| Note | New note |
|---|---|
| ![Note](docs/screenshots/note.jpg) | ![New note](docs/screenshots/new-note.jpg) |

## Docs

- [Plan](docs/STARTER.md)
- [Design notes](docs/design-notes.md)
- [Rust notes](docs/rust-notes.md)
