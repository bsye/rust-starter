# Rust starter

A containerised Dioxus fullstack scaffold for web apps, with Rust on both the server and the browser.

## Run it

Needs Docker only.

1. `docker compose up`
2. Open [localhost:8090](http://localhost:8090). The first build takes a few minutes; later edits hot reload.
3. In VS Code, run **Dev Containers: Reopen in Container** to get rust-analyzer inside the `app` container.

Checks (format, clippy for server and browser, tests):

```sh
docker compose exec app ./scripts/check.sh
```

## Docs

- [Plan](docs/STARTER.md)
- [Design notes](docs/design-notes.md)
- [Rust notes](docs/rust-notes.md)
