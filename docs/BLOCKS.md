# Plan: blocks

Goal: a Blocks page like [shadcn's](https://ui.shadcn.com/blocks). Whole screens (a dashboard, sidebars, login and
signup pages) built only from our components, each shown live in a frame next to its source. Status: phases 1 and 2
done except sidebar-09, sidebar-10 and sidebar-15; phase 3 waits on chart, table and drawer.

## What exists (checked 2026-10-29)

shadcn has 27 blocks in [`registry/new-york-v4/blocks`](https://github.com/shadcn-ui/ui/tree/main/apps/v4/registry/new-york-v4/blocks).
Their components come from `registryDependencies` in its `_registry.ts`.

| Blocks | Count | Components they need | We lack |
|---|---|---|---|
| login-01 to 05 | 5 | button, card, input, label, field | field |
| signup-01 to 05 | 5 | button, card, input, label, field | field |
| sidebar-01 to 16 | 16 | sidebar, breadcrumb, separator, label, collapsible, dropdown menu, avatar, popover, calendar, dialog, switch, card | breadcrumb |
| dashboard-01 | 1 | the sidebar set plus chart, table, drawer, tabs, select, toggle group, badge, checkbox, sheet, toast | breadcrumb, chart, table, drawer |

In the starter today:

- 43 components are installed on the `components` branch.
- `/demos/sidebar` rendered a whole-page demo inside an `iframe` on the component showcase; it is now the `sidebar-07` block.
- `src/ui/field.rs` is a small label, input and error wrapper. It is not shadcn's `Field` set (group, label,
  description, separator).
- `/sign-in` and `/sign-up` have real forms in `src/pages/auth.rs`.
- Breadcrumb and field are in phase 2 of [COMPONENTS.md](COMPONENTS.md); chart, table and drawer are in phases 3
  and 4.

## How shadcn presents a block

| Part | shadcn | Here |
|---|---|---|
| Category tabs | Featured, Sidebar, Login, Signup | The same, plus Chat, and Dashboard later |
| Preview | The block in an `iframe` of `/view/<style>/<id>` | `iframe` of `/blocks/<id>` |
| Code | Source files with a file tree | The block's Rust file, read with `include_str!` so it never drifts |
| Open in new tab | Yes | Yes, the same `/blocks/<id>` route |
| Install | `npx shadcn add <id>` | Nothing at first; later from our registry repo, if `dx components` can carry a block (not checked) |

## Decisions

| Question | Answer |
|---|---|
| Where the page lives | A new `/blocks` page with a header link. The component showcase moved to `/components`, and the home page links to both |
| Real auth pages | They keep their current look, because the benchmark runs Lighthouse on `/sign-in` against the Nuxt twin. They share the blocks' form code (`ui::credentials`), so the auth form exists once |
| Fields shadcn shows that need new backend work | Left out: full name, confirm password, forgot password, terms links, email-only sign-in |
| Cover panel (login-02, login-04, signup-02, signup-04) | A tinted panel with the app icon; no image file |
| Social and newsletter buttons | Shown disabled, with a title saying they are not set up |
| Sample data | Fictional teams and a demo user in `src/blocks/sidebar/parts/data.rs`; blocks never read the database |

## Status

| Blocks | Built | Not built, and why |
|---|---|---|
| login-01 to 05, signup-01 to 05 | All 10 | |
| chat-01 | 01 | Not a shadcn block: a chat page built from the chat components ([CHAT.md](CHAT.md)) |
| sidebar-01 to 16 | 01 to 08, 11 to 14, 16 | 09 nests two sidebars and overrides their widths, which the stock sidebar does not allow without patching it. 15 needs a left and a right sidebar in one provider, which holds a single side. 10 is buildable and next |
| dashboard-01 | | Needs chart, table and drawer (COMPONENTS.md phases 3 and 4) |

## Acceptance criteria

- GIVEN a visitor on `/blocks`, WHEN the page loads, THEN links show Featured, Sidebar, Login and Signup (Dashboard once built),
  and each block has a title, a one-line description, Preview and Code tabs and an open-in-new-tab link.
- GIVEN a block route `/blocks/<id>`, WHEN opened directly, THEN it fills the screen and works on its own.
- GIVEN Preview, THEN the block is interactive after hydration: sidebars collapse, menus open, forms validate.
- GIVEN Code, THEN the shown source is the file that renders the preview.
- GIVEN light or dark theme and a phone-width screen, THEN every block uses theme tokens only and has no horizontal
  scroll.
- GIVEN a login or signup block, WHEN submitted, THEN it signs in or creates an account with the same validation,
  errors and redirect as `/sign-in` and `/sign-up`.
- GIVEN a block route, THEN it needs no sign-in and shows no user data.

## Edge cases

- **Every frame boots the whole app.** Each `iframe` loads the full wasm: 120 MB in dev, and 3.3 MB in the last production
  build, before the components were added.
  Only the open tab's frames load, with `loading="lazy"`, so a tab with 16 sidebars does not start 16 apps at once.
  Measure memory with a full tab open.
- **One wasm bundle for every page.** Blocks grow the sign-in and notes pages too. Measure wasm size before and after.
- **Stale wasm in dev.** Frames hit the cached-wasm bug the `cache-headers` branch fixes; land that first.
- **Hot patch.** No `use_server_cached` below the root component.
- **Generated code stays generated.** Blocks live in `src/blocks/`, not `src/components/`, and only compose
  components.

## Phases

| # | Phase | Delivers |
|---|---|---|
| 1 | Page and auth blocks | `/blocks` with categories and the frame; shadcn's field set; login-01 to 05 and signup-01 to 05 |
| 2 | Sidebar blocks | Breadcrumb; sidebar-01 to 16 |
| 3 | Dashboard | dashboard-01, once chart, table and drawer exist (COMPONENTS.md phases 3 and 4) |
| 4 | Measure | Wasm size, memory with a full tab, Lighthouse on `/blocks` and `/sign-in` |
