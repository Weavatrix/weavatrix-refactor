# Refactor method index

Open only the card for the method you are about to call. Weavatrix core
analysis methods live in the separate `weavatrix` skill and are intentionally
not duplicated here.

## Complete transactional workflows

- [`rename_symbol`](tools/rename_symbol.md) — one proven rename, preview and guarded apply.
- [`rename_related_symbols`](tools/rename_related_symbols.md) — coordinated rename set, same sites as `rename_symbol`, in every indexed language.
- [`apply_edit_plan`](tools/apply_edit_plan.md) — verify and apply a compatible exact plan.
- [`rollback_last_apply`](tools/rollback_last_apply.md) — drift-safe transaction recovery.

## Plan-producing edits

- [`edit_symbol`](tools/edit_symbol.md) — symbol-range replacement or insertion.
- [`change_signature`](tools/change_signature.md) — add or remove a parameter in any indexed language.
- [`bulk_replace`](tools/bulk_replace.md) — two-stage replacement of indexed code. Comments and strings only when `include_trivia` is true.
- [`organize_imports`](tools/organize_imports.md) — removes a named binding that occurs once. Rust snake_case, Python from-imports, and Go path segments are included. Unrecognised syntax returns `NO_UNUSED_IMPORTS`.
- [`move_file`](tools/move_file.md) — filesystem rename plus specifier rewrites.
- [`move_symbol`](tools/move_symbol.md) — cycle review plus an edit plan.

## Advisory gates

- [`delete_readiness`](tools/delete_readiness.md) — review-required deletion verdict; no deletion. A private unused symbol can be `safe: true`.

## Cascading deletion

- [`delete_closure`](tools/delete_closure.md) — plan deletion of a symbol, endpoint, or file and the callees only that closure uses. `scope:"exclusive"` leaves every other contract. `scope:"connected"` also removes direct referrers. A repository returns `BLOCKED`. A module, package, or symbol with no source span returns `NOT_A_SYMBOL`. Apply the returned plan with `apply_edit_plan`.
