# Limitations

Sentio is an **AST-based static scanner** for Anchor/Solana Rust source. It does **not** execute programs, expand all macros, or prove cryptographic properties.

## What Sentio can see

- `#[account(...)]` constraints and common equivalents (`token::mint`, custom `constraint = …`)
- Instruction bodies: guards, CPI patterns, writes, basic field usage (`.key()`, data/lamports, seeds)

## What Sentio cannot see (by design)

| Out of scope | Why | What to do |
|--------------|-----|------------|
| **ZK / Groth16 / proof public inputs** | Proof binding is not in the Rust AST | `/// CHECK:`, `// sentio-ignore SWxxx`, or baseline |
| **Checks only in another program (CPI callee)** | Cross-program analysis not supported | Ignore / baseline; document trust in the other program |
| **Runtime-only values** | No full const-eval / symbolic execution | Prefer checked math and explicit guards |
| **Off-chain indexers / intent** | Cannot know your indexer contract | Use `emit!` or structured `msg!` if you want SW027 quiet |

## UncheckedAccount

Using `UncheckedAccount` is allowed. Sentio flags **missing visible guards** (owner / address / identity usage), not the type name.

- **Safe (visible):** `constraint = x.key() == config.x`, seed-only / `.key()`-only identity, `owner =` / `address =`
- **Still flagged:** data use (e.g. `try_borrow_data`) with no owner/address guard
- **Not auto-trusted:** “integrity comes from a ZK proof” without a check we can parse

## SW025 guard analysis

- **Mutation through `&mut self` methods is invisible.** Passing `&mut x` to a function is treated as a mutation, but a method call like `x.normalize()` that takes `&mut self` cannot be recognized without type resolution. A guard established before such a call survives it; if the method can set `x` back to `None`, the unwrap is not reported. Method names `take` / `replace` / `swap` are handled explicitly.
- **Slice length bounds are file-wide, not per variable.** A `require!(x.len() >= …)` lowers the length floor for every `.get(N)` / `.last()` in the file — Anchor's `remaining_accounts` convention usually means one invariant, but a file with two independent slices of different lengths can quiet an out-of-bounds unwrap on the shorter one. The smallest bound in the file wins, so a stronger require never quiets an index past a weaker one.

## False positives

Report with rule id + snippet: GitHub issues or Discord `#false-positives`. Prefer a PR with a regression fixture when fixing FPs.
