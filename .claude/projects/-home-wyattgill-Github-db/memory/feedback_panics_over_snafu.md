---
name: Use panics not snafu for now
description: User prefers panics over snafu error handling during early development phase
type: feedback
---

Use panics for error handling, not snafu Result types.

**Why:** The codebase is in early foundation phase with zero users. Typed error handling adds friction without payoff yet.

**How to apply:** Don't add snafu or Result-based error handling unless the user explicitly asks for it. Match the existing panic style.
