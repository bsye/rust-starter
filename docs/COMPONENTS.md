# Plan: every component on the home page

The home page shows every component the starter can use, at parity with [shadcn/ui](https://ui.shadcn.com/docs/components).
Components [dioxus-components](https://github.com/DioxusLabs/components) provides come from `dx components add`; the
ones it lacks we build in `src/ui/` on the same theme tokens. Status: planned, not started.

## What exists (checked 2026-10-29)

| Set | Count | Source |
|---|---|---|
| Installed in the starter | 8 | `src/components/`: alert dialog, button, card, input, label, pagination, skeleton, textarea |
| dioxus-components upstream | 45 | [`preview/src/components`](https://github.com/DioxusLabs/components/tree/main/preview/src/components) at `9a75825` (2026-09-08) |
| shadcn/ui | 61 | [`registry/new-york-v4/ui`](https://github.com/shadcn-ui/ui/tree/main/apps/v4/registry/new-york-v4/ui) |

### shadcn/ui against dioxus-components

| Group | Components |
|---|---|
| Upstream has it, not installed (30) | accordion, aspect ratio, avatar, badge, calendar, checkbox, collapsible, combobox, context menu, dialog, dropdown menu, form, hover card, item, menubar, popover, progress, radio group, scroll area, select, separator, sheet, sidebar, slider, switch, tabs, toast (shadcn's sonner), toggle, toggle group, tooltip |
| Missing upstream, small (10) | alert, breadcrumb, button group, empty, field, input group, kbd, native select, spinner, table |
| Missing upstream, large (7) | carousel, chart, command, drawer, input OTP, navigation menu (upstream has a simpler navbar), resizable |
| Missing upstream, chat UI (6) | attachment, bubble, direction, marker, message, message scroller |
| Dioxus only (7) | color picker, date picker, drag and drop list, navbar, tag group, toolbar, virtual list |

## Blockers found

- **`dx components` fails in the 0.8 image**: `dx components list` stops with "there is no TLS stream available"
  while downloading the registry. Adding anything needs this fixed first.
- **Old primitives pin**: `dioxus-primitives` is pinned to `9db1d9a`, older than the upstream head; new components may
  need the newer primitives, which means re-checking the 8 installed ones and the Textarea patch in
  [design-notes.md](design-notes.md).

## Acceptance criteria

- GIVEN a visitor on `/`, WHEN the page loads, THEN every component appears once, in sections by category (forms,
  overlays, navigation, data display, feedback, layout), with a contents list that jumps to each section.
- GIVEN light or dark theme, THEN every component uses the theme's tokens only, and both themes look right.
- GIVEN a phone-width screen, THEN the page has no horizontal scroll and every component stays usable.
- GIVEN an interactive component (dialog, menu, select, toast, tabs), WHEN used with mouse or keyboard after
  hydration, THEN it behaves like its shadcn counterpart.
- GIVEN a component missing upstream, THEN ours lives in `src/ui/`, follows shadcn's names, variants and markup, and
  any logic has a test.
- GIVEN a hot patch, THEN the page does not crash: no `use_server_cached` below the root component.

## Edge cases

- **The benchmark edits the home heading.** `bench/scripts/devloop.mjs` changes the text "Rust starter" in
  `src/pages/home.rs` to time markup changes. Keep that heading, or move the metric to another file in the same change.
- **One wasm bundle for every page.** Each component added grows the download for the sign-in and notes pages too.
  Measure wasm size and Lighthouse before and after; the benchmark tracks page weight.
- **Generated code stays generated.** `src/components/` is `dx` output; every new `style.css` goes into
  `COMPONENT_STYLES` (a test enforces it).

## Phases

| # | Phase | Delivers |
|---|---|---|
| 0 | Unblock | `dx components` working in the image, primitives pin updated, the 8 installed components re-checked |
| 1 | Upstream set | The 30 missing upstream components added, and the home page with sections and contents |
| 2 | Small gaps | The 10 small missing components in `src/ui/` |
| 3 | Large gaps | Carousel, chart, command, drawer, input OTP, navigation menu, resizable: build each, or decide to skip |
| 4 | Measure | Wasm size and Lighthouse before and after, the benchmark rerun |

## Open questions

1. Does the showcase replace the current home page, or does the current intro stay at the top?
2. Are the six chat components (attachment, bubble, direction, marker, message, message scroller) in scope?
3. Chart: shadcn builds it on Recharts (JavaScript). Build charts in Rust, or allow a JavaScript chart library?
4. Do the seven Dioxus-only components (color picker, date picker, ...) also go on the page?
