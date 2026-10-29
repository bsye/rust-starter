# Plan: expand the Dioxus component library

Goal: every [shadcn/ui](https://ui.shadcn.com/docs/components) component exists for Dioxus, and the starter's `/components`
page shows all of them. What [dioxus-components](https://github.com/DioxusLabs/components) lacks we build in its format, in
our own registry repo, so anyone can install it with `dx components add --git`. Status: phase 1 done; phase 2 next.

## Decisions

| Question | Answer |
|---|---|
| Where the showcase lives | `/components`, linked from the header; the home page links to it and to the blocks |
| Chat components | In scope, all six |
| Charts | Built in Rust; no prebuilt Rust charts exist for Dioxus |
| Dioxus-only components | On the page too |
| Where new components live | A public registry repo under `bsye`, in the dioxus-components format, using the same `dioxus-primitives` pin |
| The TLS bug in `dx components` | Worked around with a local clone, not reported |

## What exists (checked 2026-10-29)

| Set | Count | Source |
|---|---|---|
| Installed in the starter | 8 | `src/components/`: alert dialog, button, card, input, label, pagination, skeleton, textarea |
| dioxus-components for Dioxus 0.8 alpha | 44 | branch [`dioxus-v0.8.0-alpha.3`](https://github.com/DioxusLabs/components/tree/dioxus-v0.8.0-alpha.3), the exact commit our `dioxus-primitives` pin uses; `main` (Dioxus 0.7) also has tag group |
| shadcn/ui | 61 | [`registry/new-york-v4/ui`](https://github.com/shadcn-ui/ui/tree/main/apps/v4/registry/new-york-v4/ui) |

### shadcn/ui against dioxus-components

| Group | Components |
|---|---|
| Upstream has it, not installed (30) | accordion, aspect ratio, avatar, badge, calendar, checkbox, collapsible, combobox, context menu, dialog, dropdown menu, form, hover card, item, menubar, popover, progress, radio group, scroll area, select, separator, sheet, sidebar, slider, switch, tabs, toast (shadcn's sonner), toggle, toggle group, tooltip |
| Missing upstream, small (10) | alert, breadcrumb, button group, empty, field, input group, kbd, native select, spinner, table |
| Missing upstream, large (7) | carousel, chart, command, drawer, input OTP, navigation menu (upstream has a simpler navbar), resizable |
| Missing upstream, chat UI (6) | attachment, bubble, direction, marker, message, message scroller |
| Dioxus only (7) | color picker, date picker, drag and drop list, navbar, tag group (not on the 0.8 branch yet), toolbar, virtual list |

### Upstream activity to build on, not duplicate

| Item | State |
|---|---|
| Table | Open PR [#254](https://github.com/DioxusLabs/components/pull/254) |
| Input OTP | Open PR [#255](https://github.com/DioxusLabs/components/pull/255), by a maintainer |
| Chart, breadcrumb, command, data table, tree view | Requested in [#9](https://github.com/DioxusLabs/components/issues/9), [#73](https://github.com/DioxusLabs/components/issues/73), [#75](https://github.com/DioxusLabs/components/issues/75), [#244](https://github.com/DioxusLabs/components/issues/244), [#251](https://github.com/DioxusLabs/components/issues/251) |
| Dioxus 0.8 on `main` | Open PR [#278](https://github.com/DioxusLabs/components/pull/278) |
| A styled library published to crates.io | Open PR [#259](https://github.com/DioxusLabs/components/pull/259); could change how components are installed |

## Blocker and workaround

`dx components` 0.8.0-alpha.3 cannot download any registry over https ("there is no TLS stream available"). A registry
cloned with the system `git` works: `dx components add --git file:///<clone>`. A script does the clone at the pinned
commit, so adding a component stays one command.

## Acceptance criteria

- GIVEN a visitor on `/components`, WHEN the page loads, THEN every component appears once, in sections by category (forms,
  overlays, navigation, data display, feedback, layout, charts, chat), with a contents list that jumps to each section.
- GIVEN light or dark theme, THEN every component uses the theme's tokens only, and both themes look right.
- GIVEN a phone-width screen, THEN the page has no horizontal scroll and every component stays usable.
- GIVEN an interactive component (dialog, menu, select, toast, tabs, command, carousel), WHEN used with mouse or
  keyboard after hydration, THEN it behaves like its shadcn counterpart.
- GIVEN a new component, THEN it follows the dioxus-components layout (behaviour in a primitive when it has any,
  styling in the component with its `component.json`), shadcn's names and variants, and has tests for its logic.
- GIVEN a chart, THEN it renders as SVG on the server and in the browser, with axes, a legend and tooltips, from data
  passed as props.
- GIVEN a hot patch, THEN the page does not crash: no `use_server_cached` below the root component.

## Edge cases

- **The benchmark edits the home heading.** `bench/scripts/devloop.mjs` changes "Rust starter" in
  `src/pages/home.rs`. Keep that heading on the home page, or move the metric in the same change.
- **One wasm bundle for every page.** Every component added grows the sign-in and notes pages too. Measure wasm size
  and Lighthouse before and after.
- **Generated code stays generated.** `src/components/` is `dx` output; every new `style.css` goes into
  `COMPONENT_STYLES` (a test enforces it).

## Phases

| # | Phase | Delivers |
|---|---|---|
| 1 | Upstream set | The 35 components from the 0.8 registry we did not have (all but the empty `form`), installed; the showcase with sections and contents |
| 2 | Registry repo and small gaps | The registry repo with checks, then alert, breadcrumb, button group, empty, field, input group, kbd, native select, spinner |
| 3 | Large gaps | Carousel, command, drawer, navigation menu, resizable; table and input OTP from their open PRs |
| 4 | Charts | Area, bar, line, pie and radial charts in SVG, matching shadcn's chart examples |
| 5 | Chat | Attachment, bubble, direction, marker, message, message scroller |
| 6 | Measure | Wasm size and Lighthouse before and after, the benchmark rerun |
