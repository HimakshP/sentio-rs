# sentio report

## Summary

- **Total:** 190
- **Critical:** 6
- **High:** 8
- **Medium:** 176
- **Low:** 0
- **Files scanned / parsed:** 86 / 86

### By rule

| Count | Rule | Title |
|------:|------|-------|
| 3 | `SW003` | Arbitrary CPI target |
| 3 | `SW006` | Type cosplay — missing discriminator check |
| 1 | `SW011` | AccountInfo used as data account |
| 2 | `SW013` | PDA seed references unvalidated account |
| 1 | `SW014` | PDA bump may not be canonical |
| 4 | `SW024` | Division by zero |
| 176 | `SW025` | unwrap() / expect() in instruction handler |

## Findings

### 1. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/proxy_swap_processor.rs:86:36`
- **Matched because:** `.unwrap()` on `source_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 2. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/proxy_swap_processor.rs:87:36`
- **Matched because:** `.unwrap()` on `source_token_sa . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 3. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/proxy_swap_processor.rs:122:28`
- **Matched because:** `.unwrap()` on `sa_authority . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 4. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/proxy_swap_processor.rs:123:41`
- **Matched because:** `.unwrap()` on `destination_token_sa . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 5. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/proxy_swap_processor.rs:124:41`
- **Matched because:** `.unwrap()` on `destination_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 6. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:128:41`
- **Matched because:** `.unwrap()` on `destination_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 7. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:134:17`
- **Matched because:** `.unwrap()` on `destination_token_sa . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 8. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:179:28`
- **Matched because:** `.unwrap()` on `sa_authority . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 9. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:184:42`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 10. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:197:44`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 11. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:213:35`
- **Matched because:** `.unwrap()` on `source_token_sa . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 12. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:214:40`
- **Matched because:** `.unwrap()` on `source_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 13. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:218:42`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 14. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:234:44`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 15. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:303:42`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 16. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:315:44`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 17. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:333:40`
- **Matched because:** `.unwrap()` on `destination_token_sa . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 18. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:334:45`
- **Matched because:** `.unwrap()` on `destination_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 19. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:338:42`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 20. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:353:44`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 21. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:391:28`
- **Matched because:** `.unwrap()` on `trim_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 22. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:399:30`
- **Matched because:** `.unwrap()` on `charge_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 23. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:446:40`
- **Matched because:** `.unwrap()` on `destination_token_sa . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 24. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:447:45`
- **Matched because:** `.unwrap()` on `destination_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 25. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:504:40`
- **Matched because:** `.unwrap()` on `destination_token_sa . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 26. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:505:45`
- **Matched because:** `.unwrap()` on `destination_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 27. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:609:36`
- **Matched because:** `.unwrap()` on `sa_authority . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 28. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:715:21`
- **Matched because:** `.unwrap()` on `destination_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 29. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_tob_processor.rs:721:32`
- **Matched because:** `.unwrap()` on `sa_authority . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 30. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:106:42`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 31. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:115:44`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 32. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:126:40`
- **Matched because:** `.unwrap()` on `source_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 33. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:130:42`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 34. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:146:44`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 35. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:186:41`
- **Matched because:** `.unwrap()` on `destination_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 36. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:204:42`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 37. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:212:44`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 38. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:224:42`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 39. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/processor/swap_toc_processor.rs:239:44`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 40. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/stabble.rs:120:29`
- **Matched because:** `.unwrap()` on `(Some (amount_in)) . try_to_vec ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 41. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfunamm.rs:95:25`
- **Matched because:** `.unwrap()` on `account_infos . get (1)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 42. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfunamm.rs:112:25`
- **Matched because:** `.unwrap()` on `account_infos . get (1)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 43. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfunamm.rs:113:21`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 44. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfunamm.rs:228:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 45. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/manifest.rs:15:25`
- **Matched because:** `.unwrap()` on `account_infos . get (0)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 46. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/manifest.rs:32:25`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 47. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/manifest.rs:33:29`
- **Matched because:** `.unwrap()` on `account_infos . get (0)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 48. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/manifest.rs:246:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 49. `SW003` — Arbitrary CPI target

- **Severity:** critical
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/common.rs:192:12`
- **Matched because:** CPI call `invoke` in `execute_instruction` has no preceding program key validation; an attacker can supply a malicious CPI target.
- **Guidance:** Add require!(program.key() == expected::ID, ...) before the CPI, use Program<'info, T>, or an allowlist for external programs (royalties, hooks).

### 50. `SW003` — Arbitrary CPI target

- **Severity:** critical
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/common.rs:194:12`
- **Matched because:** CPI call `invoke_signed` in `execute_instruction` has no preceding program key validation; an attacker can supply a malicious CPI target.
- **Guidance:** Add require!(program.key() == expected::ID, ...) before the CPI, use Program<'info, T>, or an allowlist for external programs (royalties, hooks).

### 51. `SW003` — Arbitrary CPI target

- **Severity:** critical
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/common.rs:197:8`
- **Matched because:** CPI call `invoke_signed` in `execute_instruction` has no preceding program key validation; an attacker can supply a malicious CPI target.
- **Guidance:** Add require!(program.key() == expected::ID, ...) before the CPI, use Program<'info, T>, or an allowlist for external programs (royalties, hooks).

### 52. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/common.rs:194:55`
- **Matched because:** `.unwrap()` on `owner_seeds` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 53. `SW024` — Division by zero

- **Severity:** high
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sanctum.rs:854:8`
- **Matched because:** `std :: mem :: size_of :: < T > ()` used as divisor in `%` without a zero-check; if zero at runtime the transaction will panic
- **Guidance:** Use checked_div() or checked_rem() and handle the None case, or add require!(divisor != 0, ...) before the operation.

### 54. `SW024` — Division by zero

- **Severity:** high
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sanctum.rs:861:15`
- **Matched because:** `std :: mem :: size_of :: < T > ()` used as divisor in `/` without a zero-check; if zero at runtime the transaction will panic
- **Guidance:** Use checked_div() or checked_rem() and handle the None case, or add require!(divisor != 0, ...) before the operation.

### 55. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sanctum.rs:871:27`
- **Matched because:** `.unwrap()` on `lst_states_acc . try_borrow_data ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 56. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sanctum.rs:872:27`
- **Matched because:** `.unwrap()` on `try_lst_state_list (& * lst_states_data)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 57. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:80:21`
- **Matched because:** `.unwrap()` on `account_infos . get (12)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 58. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:81:36`
- **Matched because:** `.unwrap()` on `account_infos . get (11)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 59. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:82:29`
- **Matched because:** `.unwrap()` on `account_infos . get (8)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 60. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:83:25`
- **Matched because:** `.unwrap()` on `account_infos . get (0)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 61. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:128:41`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 62. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:129:25`
- **Matched because:** `.unwrap()` on `account_infos . get (0)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 63. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:130:29`
- **Matched because:** `.unwrap()` on `account_infos . get (8)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 64. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:136:18`
- **Matched because:** `.unwrap()` on `owner_seeds` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 65. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit.rs:228:9`
- **Matched because:** `.unwrap()` on `payer . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 66. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:102:36`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 67. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:103:29`
- **Matched because:** `.unwrap()` on `account_infos . get (11)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 68. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:104:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 69. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:140:25`
- **Matched because:** `.unwrap()` on `account_infos . get (14)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 70. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:141:29`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 71. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:424:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 72. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:555:41`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 73. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:556:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 74. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:557:29`
- **Matched because:** `.unwrap()` on `account_infos . get (10)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 75. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun2.rs:562:18`
- **Matched because:** `.unwrap()` on `owner_seeds` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 76. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/raydium_launchpad.rs:16:25`
- **Matched because:** `.unwrap()` on `account_infos . get (0)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 77. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/raydium_launchpad.rs:33:25`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 78. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/raydium_launchpad.rs:34:29`
- **Matched because:** `.unwrap()` on `account_infos . get (0)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 79. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/raydium_launchpad.rs:219:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 80. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:104:36`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 81. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:105:29`
- **Matched because:** `.unwrap()` on `account_infos . get (8)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 82. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:106:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 83. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:141:25`
- **Matched because:** `.unwrap()` on `account_infos . get (account_infos . len () - 2)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 84. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:142:29`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 85. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:236:26`
- **Matched because:** `.unwrap()` on `(amount_in as u128) . checked_mul (virtual_token_reserves as u128) . unwrap () . checked_div ((virtual_sol_reserves as u128) . checked_add (amount_in as u128) . unwrap ())` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 86. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:297:22`
- **Matched because:** `.unwrap()` on `swap_accounts . cal_token_amount_out (real_amount_in . checked_sub (1) . unwrap () , virtual_token_reserves , virtual_sol_reserves ,)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 87. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:345:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 88. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:375:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 89. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:391:41`
- **Matched because:** `.unwrap()` on `account_infos . get (account_infos . len () - 2)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 90. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:392:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 91. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:393:29`
- **Matched because:** `.unwrap()` on `account_infos . get (9)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 92. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:394:21`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 93. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:399:18`
- **Matched because:** `.unwrap()` on `owner_seeds` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 94. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:506:13`
- **Matched because:** `.unwrap()` on `(token_amount_in as u128) . checked_mul (virtual_sol_reserves as u128)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 95. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:508:13`
- **Matched because:** `.unwrap()` on `(virtual_token_reserves as u128) . checked_add (token_amount_in as u128)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 96. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:510:30`
- **Matched because:** `.unwrap()` on `numerator . checked_div (denominator)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 97. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:569:30`
- **Matched because:** `.unwrap()` on `sol_amount_out . checked_sub (total_fee)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 98. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:607:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 99. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:640:9`
- **Matched because:** `.unwrap()` on `fee . checked_add (compute_fee (amount , creator_fee_basis_points))` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 100. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:651:14`
- **Matched because:** `.unwrap()` on `amount . checked_mul (fee_basis_points)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 101. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:655:5`
- **Matched because:** `.unwrap()` on `a . checked_add (b . checked_sub (1) . unwrap ()) . unwrap () . checked_div (b)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 102. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:677:8`
- **Matched because:** `.unwrap()` on `compute_fee (amount , protocol_fee_bps) . checked_add (creator_fee)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 103. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:708:26`
- **Matched because:** `.unwrap()` on `fee_config` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 104. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:884:26`
- **Matched because:** `.unwrap()` on `(amount_in as u128) . checked_mul (virtual_token_reserves as u128) . unwrap () . checked_div ((virtual_sol_reserves as u128) . checked_add (amount_in as u128) . unwrap ())` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 105. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:945:22`
- **Matched because:** `.unwrap()` on `swap_accounts . cal_token_amount_out (real_amount_in . checked_sub (1) . unwrap () , virtual_token_reserves , virtual_sol_reserves ,)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 106. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:994:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 107. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:1089:13`
- **Matched because:** `.unwrap()` on `(token_amount_in as u128) . checked_mul (virtual_sol_reserves as u128)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 108. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:1091:13`
- **Matched because:** `.unwrap()` on `(virtual_token_reserves as u128) . checked_add (token_amount_in as u128)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 109. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:1093:30`
- **Matched because:** `.unwrap()` on `numerator . checked_div (denominator)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 110. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:1152:30`
- **Matched because:** `.unwrap()` on `sol_amount_out . checked_sub (total_fee)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 111. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/pumpfun.rs:1191:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 112. `SW024` — Division by zero

- **Severity:** high
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/openbookv2.rs:268:29`
- **Matched because:** `base_lot_size` used as divisor in `/` without a zero-check; if zero at runtime the transaction will panic
- **Guidance:** Use checked_div() or checked_rem() and handle the None case, or add require!(divisor != 0, ...) before the operation.

### 113. `SW024` — Division by zero

- **Severity:** high
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/openbookv2.rs:291:19`
- **Matched because:** `base_lot_size` used as divisor in `/` without a zero-check; if zero at runtime the transaction will panic
- **Guidance:** Use checked_div() or checked_rem() and handle the None case, or add require!(divisor != 0, ...) before the operation.

### 114. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/openbookv2.rs:144:25`
- **Matched because:** `.unwrap()` on `i64 :: try_from (amount_in)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 115. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/openbookv2.rs:154:41`
- **Matched because:** `.unwrap()` on `i64 :: try_from (amount_in)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 116. `SW006` — Type cosplay — missing discriminator check

- **Severity:** critical
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/numeraire.rs:113:28`
- **Matched because:** `Pubkey::try_from_slice` called without skipping the 8-byte discriminator; an attacker can pass an account of a different type with the same byte layout
- **Guidance:** Use Account<'info, T> (Anchor checks the discriminator for you), or pass &account.data.borrow()[8..] to skip the discriminator bytes manually.

### 117. `SW006` — Type cosplay — missing discriminator check

- **Severity:** critical
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/numeraire.rs:114:28`
- **Matched because:** `Pubkey::try_from_slice` called without skipping the 8-byte discriminator; an attacker can pass an account of a different type with the same byte layout
- **Guidance:** Use Account<'info, T> (Anchor checks the discriminator for you), or pass &account.data.borrow()[8..] to skip the discriminator bytes manually.

### 118. `SW006` — Type cosplay — missing discriminator check

- **Severity:** critical
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/numeraire.rs:115:28`
- **Matched because:** `Pubkey::try_from_slice` called without skipping the 8-byte discriminator; an attacker can pass an account of a different type with the same byte layout
- **Guidance:** Use Account<'info, T> (Anchor checks the discriminator for you), or pass &account.data.borrow()[8..] to skip the discriminator bytes manually.

### 119. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:56:36`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 120. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:57:29`
- **Matched because:** `.unwrap()` on `account_infos . get (9)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 121. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:58:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 122. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:93:25`
- **Matched because:** `.unwrap()` on `account_infos . get (16)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 123. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:94:29`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 124. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:136:41`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 125. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:137:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 126. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:138:29`
- **Matched because:** `.unwrap()` on `account_infos . get (9)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 127. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:144:18`
- **Matched because:** `.unwrap()` on `owner_seeds` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 128. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/sugar_money.rs:291:24`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 129. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:105:36`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 130. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:106:29`
- **Matched because:** `.unwrap()` on `account_infos . get (11)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 131. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:107:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 132. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:143:25`
- **Matched because:** `.unwrap()` on `account_infos . get (14)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 133. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:144:29`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 134. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:433:9`
- **Matched because:** `.unwrap()` on `payer` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 135. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:564:41`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 136. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:565:25`
- **Matched because:** `.unwrap()` on `account_infos . get (6)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 137. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:566:29`
- **Matched because:** `.unwrap()` on `account_infos . get (10)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 138. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/boopfun.rs:571:18`
- **Matched because:** `.unwrap()` on `owner_seeds` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 139. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:74:21`
- **Matched because:** `.unwrap()` on `account_infos . get (12)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 140. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:75:36`
- **Matched because:** `.unwrap()` on `account_infos . get (11)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 141. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:76:29`
- **Matched because:** `.unwrap()` on `account_infos . get (8)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 142. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:77:25`
- **Matched because:** `.unwrap()` on `account_infos . get (0)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 143. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:122:41`
- **Matched because:** `.unwrap()` on `account_infos . last ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 144. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:123:25`
- **Matched because:** `.unwrap()` on `account_infos . get (0)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 145. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:124:29`
- **Matched because:** `.unwrap()` on `account_infos . get (8)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 146. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:130:18`
- **Matched because:** `.unwrap()` on `owner_seeds` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 147. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/adapters/moonit2.rs:222:9`
- **Matched because:** `.unwrap()` on `payer . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 148. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/token.rs:209:20`
- **Matched because:** `.unwrap()` on `token_sa . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 149. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/token.rs:212:13`
- **Matched because:** `.unwrap()` on `associated_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 150. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/token.rs:216:28`
- **Matched because:** `.unwrap()` on `sa_authority . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 151. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/token.rs:218:33`
- **Matched because:** `.unwrap()` on `system_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 152. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/token.rs:219:32`
- **Matched because:** `.unwrap()` on `token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 153. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/token.rs:249:9`
- **Matched because:** `.unwrap()` on `transfer_fee_config . calculate_epoch_fee (Clock :: get () ? . epoch , pre_fee_amount)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 154. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:20:9`
- **Matched because:** `.unwrap()` on `u64 :: try_from (u128 :: from (amount) . checked_mul (commission_rate as u128) . ok_or (ErrorCode :: CalculationError) ? . checked_div (COMMISSION_DENOMINATOR_V2 as u128 - commission_rate as u128) . ok_or (ErrorCode :: CalculationError) ? ,)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 155. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:29:9`
- **Matched because:** `.unwrap()` on `u64 :: try_from (u128 :: from (amount) . checked_mul (commission_rate as u128) . ok_or (ErrorCode :: CalculationError) ? . checked_div (COMMISSION_DENOMINATOR_V2 as u128) . ok_or (ErrorCode :: CalculationError) ? ,)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 156. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:39:65`
- **Matched because:** `.unwrap()` on `platform_fee_rate` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 157. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:40:33`
- **Matched because:** `.unwrap()` on `platform_fee_rate` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 158. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:45:9`
- **Matched because:** `.unwrap()` on `u64 :: try_from (u128 :: from (commission_amount) . checked_mul (platform_fee_rate as u128) . ok_or (ErrorCode :: CalculationError) ? . checked_div (PLATFORM_FEE_DENOMINATOR_V3 as u128) . ok_or (ErrorCode :: CalculationError) ? ,)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 159. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:75:31`
- **Matched because:** `.unwrap()` on `trim_rate` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 160. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:78:21`
- **Matched because:** `.unwrap()` on `trim_rate` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 161. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:81:22`
- **Matched because:** `.unwrap()` on `u64 :: try_from (u128 :: from (amount) . saturating_mul (trim_rate as u128) . saturating_div (TRIM_DENOMINATOR_V2 as u128) ,)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 162. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:98:33`
- **Matched because:** `.unwrap()` on `charge_rate` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 163. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:99:27`
- **Matched because:** `.unwrap()` on `charge_rate` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 164. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:102:29`
- **Matched because:** `.unwrap()` on `u64 :: try_from (u128 :: from (trim_amount) . saturating_mul (charge_rate as u128) . saturating_div (TRIM_DENOMINATOR_V2 as u128) ,)` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 165. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:184:12`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 166. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/utils/fee.rs:189:12`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 167. `SW013` — PDA seed references unvalidated account

- **Severity:** high
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/create_token_account_with_seed.rs:24:1`
- **Matched because:** PDA `token_account` uses `owner` as a seed, but `owner` is an unvalidated AccountInfo — an attacker can supply any account as the seed input
- **Guidance:** Add `owner`, `address`, or `signer` constraint to `owner`, or change its type to Signer<'info> or Program<'info, T>.

### 168. `SW011` — AccountInfo used as data account

- **Severity:** high
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:35:1`
- **Matched because:** Account `sa_authority` is typed as `AccountInfo` but has data-account constraints; use `Account<'info, T>` to enforce owner and discriminator checks.
- **Guidance:** Define a typed account struct and use Account<'info, YourStruct> so Anchor validates the owner program and discriminator on deserialization.

### 169. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:56:43`
- **Matched because:** `.unwrap()` on `destination_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 170. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:68:14`
- **Matched because:** `.unwrap()` on `destination_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 171. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:69:14`
- **Matched because:** `.unwrap()` on `ctx . accounts . token_mint . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 172. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:70:14`
- **Matched because:** `.unwrap()` on `ctx . accounts . token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 173. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:71:14`
- **Matched because:** `.unwrap()` on `ctx . accounts . associated_token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 174. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:78:30`
- **Matched because:** `.unwrap()` on `ctx . accounts . destination_token_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 175. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:144:36`
- **Matched because:** `.unwrap()` on `ctx . accounts . source_token_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 176. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:146:42`
- **Matched because:** `.unwrap()` on `ctx . accounts . destination_token_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 177. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:148:22`
- **Matched because:** `.unwrap()` on `ctx . accounts . token_mint . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 178. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/claim.rs:149:25`
- **Matched because:** `.unwrap()` on `ctx . accounts . token_program . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 179. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/wrap_unwrap_v3.rs:550:38`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 180. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/wrap_unwrap_v3.rs:562:40`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 181. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/wrap_unwrap_v3.rs:574:38`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 182. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/wrap_unwrap_v3.rs:582:40`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 183. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/wrap_unwrap_v3.rs:632:38`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 184. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/wrap_unwrap_v3.rs:647:40`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 185. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/wrap_unwrap_v3.rs:662:38`
- **Matched because:** `.unwrap()` on `commission_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 186. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/wrap_unwrap_v3.rs:677:40`
- **Matched because:** `.unwrap()` on `platform_fee_account . as_ref ()` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 187. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/swap_v3.rs:78:50`
- **Matched because:** `.unwrap()` on `trim_rate` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 188. `SW025` — unwrap() / expect() in instruction handler

- **Severity:** medium
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/swap_v3.rs:273:50`
- **Matched because:** `.unwrap()` on `trim_rate` will panic on None/Err; use `?` or `.ok_or(ErrorCode::...)?` instead
- **Guidance:** Use `?` to propagate errors or `.ok_or(ErrorCode::Foo)?` to convert Option to a typed program error.

### 189. `SW014` — PDA bump may not be canonical

- **Severity:** high
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/proxy_swap.rs:31:1`
- **Matched because:** PDA `sa_authority` uses `bump = BUMP_SA` — verify `BUMP_SA` is the canonical bump stored on-chain rather than a user-supplied value
- **Guidance:** On init, let Anchor derive the canonical bump with just `bump` (no value). Store it in the account and reuse it with `bump = account.bump` on every subsequent instruction.

### 190. `SW013` — PDA seed references unvalidated account

- **Severity:** high
- **Location:** `/home/krak1n/Web3-DEX-Router-Solana-V1/programs/dex-solana/src/instructions/create_token_account.rs:23:1`
- **Matched because:** PDA `token_account` uses `owner` as a seed, but `owner` is an unvalidated AccountInfo — an attacker can supply any account as the seed input
- **Guidance:** Add `owner`, `address`, or `signer` constraint to `owner`, or change its type to Signer<'info> or Program<'info, T>.

