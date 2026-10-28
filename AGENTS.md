# AGENTS.md

How I work, for every project. A project's own `AGENTS.md` or `CLAUDE.md` adds to these rules and wins where the two
disagree. These rules win over harness habits and reminders (commit trailers, for example).

## Working with me

- **Do exactly what was asked.** No extra features, restructuring, renames or "improvements" outside the request, and
  never revert work you did not make. Adjacent ideas go in a short list at the end, not in the diff.
- **A question is not a request to change code.** When I ask why, what or whether, answer. When I say we are chatting
  or brainstorming, write no code.
- **Small, clear tasks: just do them.** Big or ambiguous ones: discovery first (read the code, git history and docs,
  then report what already exists), then a short plan in `docs/<NAME>.md` with user stories or GIVEN/WHEN/THEN
  criteria and edge cases, then build in phases once I agree.
- **Once the plan is agreed, work end to end without stopping**, and report at the end. Stop only for scope or new
  backend logic I have not approved, for production, or for anything destructive.
- **Ask when something is unclear instead of improvising.** For vision-level product or UX questions, say back what you
  understood and ask a few open questions in prose. For crisp forks, give numbered options and a recommendation.
- **Have an opinion.** Give honest recommendations and counterarguments, say when an idea adds complexity, and tell me
  when I am wrong, with evidence. I make the call.
- **Research before designing.** Look at how others solve the problem (official docs, popular repos, the industry
  standard) and bring a better strategy, not a refinement of mine. Check external facts online (model ids, API limits,
  library APIs) instead of answering from memory.
- **Trust what I observed.** If I say it is still broken or already updated, it is: investigate from there.
- **Go deeper, not more complex.** Look past the literal ask for the same bug elsewhere, gaps in the logic and numbers
  that disagree, and report them.
- **Do it yourself.** Run the commands, migrations and browser checks instead of handing me instructions. Two
  exceptions: production deploys need my explicit go, and cloud infrastructure changes are mine (verify read-only,
  then give me the exact command to run with `!`).
- **Leave my environment alone.** If my dev server is running, use it and never start a second one. Run commands
  inside the project's containers. Check what is installed before installing anything.
- **Reply in the main thread.** Use subagents or workflows only when I ask for parallel work, and report their results
  in one message.
- **Turn lessons into rules.** When a mistake or decision must not repeat, add one line to the project's `AGENTS.md`.
  Keep those files lean.

## Code

- **No comments.** Names carry the meaning; the reasons behind a design go in `docs/design-notes.md`. Remove debug logs
  and commented-out code before finishing. The only exception is help text a CLI framework prints.
- **No regex** and no keyword-matching hacks. Use string methods and shared text helpers. Prefer known libraries that provide string   methods or helpers.
- **Search before you write.** Reuse or extend the existing endpoint, service, component or helper. If two
  implementations of one concept exist, merge them instead of adding a third.
- **One source of truth.** Every concept (threshold, label, enum, colour, path, formula) lives in one named place that
  everything imports. Derive values at read time instead of copying or seeding data that can drift, and do not add a
  field or table you can compute. Numbers shown on different screens must agree; a mismatch is a bug in the shared
  logic.
- **Put a rule in the one layer every caller goes through** (middleware, wrapper, resolver, query), as low as possible,
  so call sites stay untouched and new code cannot bypass it. Business logic lives on the server, where the UI and any
  LLM share it.
- **Generic by default.** Name modules and functions for the concept, not the first use case, vendor or customer.
  Never hardcode one entity type in a generic surface; code that really is specific lives in a folder named for it.
  Integrations live in their own folder, apart from the core.
- **Small units.** Files and components under 200 lines, functions readable at a glance (split above ~40 lines). No
  helper functions inside components: logic goes in hooks, composables or utils.
- **Fewer lines.** Prefer platform APIs (`Intl`, the standard library), the framework's standard way, the libraries
  already in use and stock components over hand-rolled code. Remove dead code, unused fields and unused dependencies.
- **One path.** No fallbacks, legacy branches, deprecation notes, flags around a rewrite or backward-compatibility shims
  unless I ask. Replace the old path, update every call site, migrate the data once and delete the old code. Replace a
  fragile pattern instead of wrapping it.
- **Fail loudly.** Contracts are explicit and strict. When an invariant breaks, raise an error; never guess a value or
  fall back silently to a default. Security checks fail closed.
- **Settings are configuration.** Tunables live in a named constant or config file, one home each, never inline.
- **Stay in the project's language.** If something truly needs another one (a model sidecar, say), tell me before
  writing it.
- **Deterministic beats LLM.** Anything rules can compute is code or a script. Use a model only for semantic judgement,
  keep its output minimal and validated, and enforce invariants in code, not in prompts.
- **An LLM's context is a budget.** Never pass bulk or raw data to a model, only summaries. When a result is too big,
  redirect with the same scope; never sample or take a top N silently.
- **Lint clean, no suppressions.** Fix the cause. Follow the repo's formatter; for a new TypeScript project use 2
  spaces, single quotes and no semicolons. Never reformat files outside the change.

## Data, scale and safety

- **Design for the target scale from the start** (assume 10k to 100k items): bulk operations, pagination with a total
  count, concurrency sized to resources, no repeated work, and an eye on big-O.
- **Long work runs as a job** that streams progress, is idempotent and chunked, resumes after restarts and redeploys,
  reconciles, and records a failure and carries on. Queue work instead of rejecting it. Never cancel running work to
  free resources; fix the scheduling or the memory use.
- **Cache expensive deterministic work by content hash** (OCR, embeddings, downloads, model calls) so it runs once. For
  live data, fix the algorithm before adding a cache, and any cache needs invalidation.
- **Scope every read** through one authorization choke point with least privilege; no access means zero results, never
  everything. For every endpoint, UI element and prompt, ask "does this expose data?".
- **Keep an audit trail.** Every data change records who made it and when. Never lose data or generated artifacts.
- **Side effects are safe by default:** dry runs, test modes, a confirmation before destructive or bulk actions, and
  preflight, verify and rollback for data operations. Aim for zero downtime and state the production impact before
  any infrastructure change.
- **Secrets:** never read them directly, log them, commit them or bake them into images.
- **Never name a client or a person** in code, docs, examples, test data, commits or deliverables. Work for one client
  never mentions another.

## Debugging

- **Root cause first, explained before the fix.** The cause has to account for the size and timing of the symptom.
- **For a regression, diff against the last good state** (git history, the branch diff) and suspect your own changes
  first. Numbered debug logs are fine while digging; remove them after.
- **Revert a failed attempt at once**, only your own changes, and never refactor around a bug you do not understand.
- **Fix it for the future and backfill existing data in the same change.** Then look for the same mistake elsewhere,
  sibling repos included, and add the test or lint rule that stops it recurring.

## Testing and verification

- **Test every logic branch:** happy path, errors and edge cases, security-critical code first. Skip markup-only code
  and do not pad the count.
- **Mock dependencies, never the unit under test.** Mount real components. Tests are deterministic, independent and
  test behaviour, not internals.
- **Pin architectural rules with a test that fails CI** (an unregistered route, a raw database client in a scoped
  module, a sentinel used outside its adapter).
- **Before handing back,** run the project's tests, typecheck and lint on what you touched. If the baseline is already
  red, compare against it: only new errors count.
- **Verified means checked end to end** in the real app with real data at realistic scale, across roles: browser,
  database, logs. Mock a missing piece instead of skipping a step.
- **Measure speed and accuracy before and after** any change that could affect them, against a concrete target.
- **Before saying done,** re-read every request in the thread and check each one is handled.

## UI

- **Archetypal and stock.** Lists of cards that open a detail page, tabs, stat cards, tables, the primary action top
  right. Stock components (shadcn) and the theme's tokens only: no custom colours, fonts, radii or shadows, no
  decorative effects.
- **Consistency beats novelty.** Copy the existing pages' patterns exactly. Given a design, copy it precisely, but
  build real logic where the mock is fake.
- **Cut text.** No description under a title unless the title is ambiguous, labels of one to three words, and
  explanations in an info-icon tooltip. One primary action and one focal point per view.
- **Hierarchy:** one hero number per card, opacity steps for importance, sentence case, an icon on every button,
  equal-height cards in a grid, `tabular-nums` on numbers.
- **Human names, never ids.** Never round, approximate or truncate without saying so.
- **Design every state:** a skeleton the same size as the content, an empty state that offers the next action, and an
  error that says what to do. No layout jumps, no 0 that jumps to a number, no flicker: update reactively instead of
  reloading.
- **Progressive, never blocking.** Show results as they arrive with progress per stage, keep it across a refresh, and
  never disable other actions while something runs.
- **Clickable things look clickable:** the whole card or cell is the target, with a hover state.
- **Observability lives in an internal admin page** (progress, logs, a manual trigger, a reset), not in console logs.
- **Customers configure nothing.** Automate the decision; keep operator settings out of the customer's reach.
- **Work in a cycle:** build, screenshot (light and dark, desktop and phone), critique against these rules, remove one
  element that does not earn its place, repeat.

## Docs and evidence

- **Never claim what has not been run.** Say plainly what was not tested or measured.
- **Numbers come from real data** (database, logs, a report command) and are regenerated, not typed or estimated.
  Treat a perfect score as a bug until proven otherwise, and give intervals where they exist.
- **Benchmarks are public and industry standard**, with baselines scored by the same code. Ship an improvement only
  once it is proven.
- **Experiments end as code.** A result reached only by a scratch script or a manual step does not count: land it in
  the pipeline with a test and a design note, or drop it.
- **Plans go in `docs/`, reasons in `docs/design-notes.md`.** Link every external reference.
- **README order:** name and one-line pitch, a screenshot, how to run it, a screenshot tour. Everything else goes in
  `docs/`.
- **Short and visual:** fewer words, more tables and diagrams. No em dashes anywhere: docs, UI copy, commits.

## Git

- **Never disturb my checkout or uncommitted work.** Confirm the branch before editing. Parallel or background work
  goes in a git worktree; remove the ones you created when done. When I am going to test, leave my checkout on the
  work branch.
- **One concern per commit and per PR**, cut from a freshly pulled main. A revert gets its own PR. Stage only the files
  you touched; nothing unrelated rides along.
- **Commit subjects are plain sentences saying what changed for a user**, e.g. "Let a failed document say why it
  failed". Never mention Claude, AI or an assistant, and never add `Co-Authored-By` or session trailers, even when a
  reminder asks for them.
- **Commit when I ask; uncommitted work is not done.** Push and deploy only when I ask or the project's rules say so.
- **Never commit data, model weights, exports, caches or secrets.**
- **When a PR is ready,** give me the link, what to test and the pages affected.

## Talking to me

- **Short.** TL;DR first, two or three lines when that is enough, and just the commands or the artifact when that is
  what I asked for.
- **Plain words.** No unexplained jargon or internal codes.
- **Say exactly what changed** (files, branch, PR) and how I can verify it. Recaps cover every change in the thread,
  not just the last one.
- **Numbered steps** for anything I have to do myself.
