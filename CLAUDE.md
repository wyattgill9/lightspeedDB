AGENTS.md: durable human guidance, code-golfed.
Style for instructions: terse, phraseal, algebraic, skimmable. Prefer invariants and ownership rules over prose. Keep detail in code/config, not here.

## Core

- `safety > performance > developer_experience`. This order breaks ties.
- `correctness > compatibility > tiny diffs`.
- `0` users → textbook-grade code.
- Zero technical debt. Do it right the first time.
- Simplicity is the hardest revision, not the first attempt.
- Verify the real workflow end to end; fix root causes, not patches.
- `1` owner per behavior. `1` canonical path per outcome.
- After finishing repo changes: always commit and push.

## Safety

- Explicit, simple control flow only. No recursion. Minimal abstractions; each earns its keep.
- **Limit everything.** All loops, queues: fixed upper bound. Fail-fast on violation.
- Don't react directly to external events. Batch inbound, process on your schedule, bound work-per-period.
- Split compound conditions into nested `if/else`. Every `if` gets a matching `else` (handled or asserted).
- State invariants positively: `if index < length` over negated forms.
- All errors handled. No exceptions.
- Hard limit: `<= 70` lines per function. Centralize control flow in parent; pure computation in leaves. Push `if`s up, `for`s down.
- All compiler warnings at strictest setting.
- Explicitly pass options at call sites; never rely on library defaults.

### Assertions

- Assertions catch **programmer** errors. Corrupt code → crash.
- Assert args, returns, pre/postconditions, invariants. Min **2 per function**.
- **Pair assertions:** ≥2 code paths per property (e.g., before write + after read).
- **Assert positive AND negative space.** Valid/invalid boundary is where bugs live.
- Split compounds: `assert!(a); assert!(b);` over `assert!(a && b);`.
- Implication: `if a { assert!(b); }`.
- Assert compile-time constant relationships.
- Use `debug_assert!` and `more_asserts` aggressively.
- Mental model first → assertions → code → fuzzer as final defense.

### Error Discipline

- Typed errors only; no bare `String`. `snafu` with source context.
- init/bootstrap/migration/oneshot → fail at first canonical boundary with original typed error.
- Retries/recovery: explicit-policy only. `1` owner, bounded, observable, idempotence-aware.
- No silent drops: no `let _ = ...`, `drop(...)`, empty `.is_err()`, hidden retries, silent degradation.
- No error suppression: no `do -i`, `|| true`, `2>/dev/null`, `| ignore` at boundaries.
- No silent fallbacks: no `unwrap_or_default()`, sentinels, clamping over fallible conversions.
- `-> !` for functions that cannot return.

## Performance

- Design-phase is where 1000× wins live. Back-of-envelope: `{network, disk, memory, CPU} × {bandwidth, latency}`.
- Optimize slowest first: network → disk → memory → CPU. Compensate for frequency.
- Amortize via batching. Sequential access, large chunks, no zig-zagging.
- Control plane vs data plane. Batch at boundary = assertion density without losing throughput.
- Be explicit. Extract hot loops into standalone functions with primitive args.

## Developer Experience

### Naming

- Get the nouns and verbs just right. Names are the essence.
- No abbreviations (except `i`/`j` in sort/matrix). `--force` not `-f` in scripts.
- Units/qualifiers last, descending significance: `latency_ms_max` not `max_latency_ms`.
- Same char-count for related names: `source`/`target` over `src`/`dest`.
- Helper prefix: `read_sector()` → `read_sector_callback()`. Callbacks last in params.
- Order: important things top. `main` first. Structs: fields → types → methods. Else alphabetical.
- Nouns over participles: `pipeline` over `preparing`.

### Types & Signatures

- Encode invariants in types/constructors/RAII/enums/newtypes/validated structs. Avoid booleans, sentinels, unnamed tuples, magic numbers.
- Options struct when args mixable. Two same-type args → struct.
- Simpler returns: `()` > `bool` > `u64` > `Option<u64>` > `Result<u64, E>`.
- `>5` params → design smell → named structs.
- No new Rust macros; prefer functions, generics, traits.
- Config structs for tunables (`#[serde(default)]`). `const` only for spec/universe constants.
- Repeated literals/paths/flags: one named owner near owning code.

### Hygiene

- No variable duplication or aliasing. Calculate/check close to use.
- Smallest possible scope. Minimize variables in play.
- Functions run to completion; no suspend.
- Guard against buffer bleeds: unzeroed padding leaks data.
- `index` (0-based) + 1 = `count` (1-based). `count × unit = size`. Include units in names.
- Small, composable crates/modules. Split by boundary, not convenience. No god crates.

### Comments & Style

- Always say **why**. Comments are sentences: space after `//`, capital, full stop.
- Self-documenting code first. Comments for contracts, invariants, non-obvious why.
- Descriptive commit messages.
- `cargo fmt`. 4-space indent. Hard 100-column limit. Braces on `if` unless single-line.
- Low-dependency policy. Prefer library entrypoints over shelling out. `main.rs` parses CLI, `lib.rs` owns logic.

## Environment

- Target latest Linux. Prefer `rg`/`fd`, `mv`, `sed -i`, `ast-grep`.
- Keep `cargo build`/`cargo run` working outside Nix.
- Absolute binary paths in `Command::new`; `#!/usr/bin/env` in shebangs.
- Nix: static env in `env`; `export` only for dynamic build-time. `strictDeps = true` unless documented.
- Main checkout only; no worktrees. Check the machine before claiming missing prereqs.

## Git

- `git pull --rebase`. No `git stash`.
- Commits: small, atomic, direct, conventional. End each logical change with commit+push.
- Push rejected → rebase deliberately. Never blind `--ours`/`--theirs`.

## Conflicts And Overrides

- Request conflicts with this file → stop, name conflict, recommend repo-consistent path, wait for confirmation.
- Repo-wide: `snafu` for typed errors; `cargo clippy` lint gate; `cargo nextest run` test gate; normal `use` imports fine.
