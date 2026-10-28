# Nuxt starter (benchmark baseline)

The same notes app as the Rust starter, built the idiomatic Nuxt way, for the comparison in
[docs/BENCHMARK.md](../../docs/BENCHMARK.md).

1. `docker compose up --build`
2. Open [localhost:8092](http://localhost:8092) and create an account.

Checks (lint, typecheck, tests; the database tests need the stack running):

```sh
docker compose exec app npm run check
```
