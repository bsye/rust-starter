# Plan: chat components

Goal: shadcn's six chat components exist for Dioxus in our public registry,
[bsye/dioxus-registry](https://github.com/bsye/dioxus-registry), installable with `dx components add --git`, and the
starter shows them on `/components` and in a chat page block on `/blocks`. This is phase 5 of
[COMPONENTS.md](COMPONENTS.md), done before the registry's small gaps because chat was asked for first.

## Decisions

| Question | Answer |
|---|---|
| Where the components live | [bsye/dioxus-registry](https://github.com/bsye/dioxus-registry), public, in the [dioxus-components](https://github.com/DioxusLabs/components) layout and pins (Dioxus 0.8.0-alpha.3, `dioxus-primitives` rev `9db1d9a`) |
| Where behaviour lives | The registry's `primitives` crate (`dioxus-registry-primitives`): the message scroller and the direction context. Styled components only hold markup and CSS |
| Styling | CSS modules on the variables of `dx-components-theme.css`, like upstream; no Tailwind inside a component |
| Buttons inside components | Upstream's `button` (it has the `ghost`, `secondary` and `icon-xs`/`icon-sm` sizes shadcn uses), declared as a cross-registry dependency |
| Scope | direction, message, bubble, marker, attachment, message scroller; plus spinner and input group, which attachment and the chat block need |
| Message scroller | Full parity with `@shadcn/react/message-scroller`: follow-to-end, turn anchoring with a peek and tail spacer, opening position, prepend preservation, scroll commands, scrollable edges, visibility, scroll buttons |
| Where shown | A Chat section on `/components`; a Chat category on `/blocks` with one chat page |
| The chat block | A sample conversation from fictional data and a local composer that appends your message; no replies, no backend |
| Install | `scripts/components.sh` gains a registry argument; both registries are cloned with the system `git` and installed from `file://` (the `dx components` TLS bug) |

## Acceptance criteria

- GIVEN a project on Dioxus 0.8 alpha, WHEN `dx components add --git https://github.com/bsye/dioxus-registry message`
  runs (or our script, while the TLS bug stands), THEN the component lands in `src/components/message` with its CSS.
- GIVEN the registry, WHEN CI runs, THEN fmt, clippy with `-D warnings`, the tests and a `wasm32` check pass.
- GIVEN `/components`, THEN a Chat section shows every chat component's demo, in light and dark, with no horizontal
  scroll at 390 px.
- GIVEN a scroller with `auto_scroll` at the end, WHEN a message is appended, THEN the viewport follows it.
- GIVEN the reader scrolled up, WHEN a message arrives, THEN the position stays and the scroll button appears; WHEN
  the button is pressed, THEN the viewport scrolls to the end and the button hides.
- GIVEN a new turn marked `scroll_anchor`, WHEN it is appended, THEN it settles near the top with the previous turn
  peeking above, held by a tail spacer while its reply grows.
- GIVEN older messages prepended, THEN the first visible message stays where it was.
- GIVEN `default_scroll_position` start, end or last-anchor, THEN the transcript opens there, hidden until placed.
- GIVEN a visibility subscriber, THEN it gets the visible message ids in order and the current anchored turn.
- GIVEN the chat block, WHEN I type and send, THEN my message (and any attachment chips) joins the thread and the
  scroller follows it.

## Edge cases

- **Server render.** The scroller measures nothing on the server; `data-pending-scroll` hides the viewport until the
  opening position is applied in the browser.
- **Empty transcript.** No opening scroll; the pending flag clears so the empty state shows.
- **Unknown message id.** `scroll_to_message` queues the jump until the row mounts, as upstream does.
- **Several anchored turns at once while following.** Keep following the end instead of jumping to the first.
- **A turn taller than the viewport.** Held at the reading line with no spacer, never yanked to the end.
- **Right to left.** `DirectionProvider` sets the context; the components use logical properties.

## Phases

| # | Phase | Delivers |
|---|---|---|
| 1 | Registry | The repo, workspace, CI, licences, README; direction, spinner, message, bubble, marker, attachment, input group |
| 2 | Scroller | Geometry and controller in `primitives` with tests; the styled scroller and its examples |
| 3 | Starter | Script support for our registry; components installed; Chat section; chat block |
| 4 | Verify | Checks in both repos, the browser pass in light, dark, desktop and phone |

## Status

| Item | State |
|---|---|
| Registry repo | Public at [bsye/dioxus-registry](https://github.com/bsye/dioxus-registry), 22 primitive tests and 3 layout tests |
| Components | message, bubble (7 variants, reactions, `r#as`), marker (3 variants, icon, shimmer), attachment (5 states, 3 sizes, 2 orientations, media, actions, trigger, group), message scroller (full policy, button, 3 hooks), input group, spinner, direction |
| Starter | Installed through `scripts/components.sh`; Chat section on `/components`; `chat-01` on `/blocks` |
| CI on GitHub | Workflow written; pushing it needs a token with the `workflow` scope |

Next steps that need approval: a real chat (persisted threads, other people, replies) needs backend logic.
