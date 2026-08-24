# Refactor method index

Open only the card for the method you are about to call. Weavatrix core
analysis methods live in the separate `weavatrix` skill and are intentionally
not duplicated here.

## Complete transactional workflows

- [`rename_symbol`](tools/rename_symbol.md) — one proven rename, preview and guarded apply.
- [`rename_related_symbols`](tools/rename_related_symbols.md) — coordinated JS/TS rename set.
- [`apply_edit_plan`](tools/apply_edit_plan.md) — verify and apply a compatible exact plan.
- [`rollback_last_apply`](tools/rollback_last_apply.md) — drift-safe transaction recovery.

## Plan-producing edits

- [`edit_symbol`](tools/edit_symbol.md) — symbol-range replacement or insertion.
- [`change_signature`](tools/change_signature.md) — partial JS/TS parameter surgery.
- [`bulk_replace`](tools/bulk_replace.md) — two-stage occurrence-selective replacement.
- [`organize_imports`](tools/organize_imports.md) — conservative JS/TS named-import cleanup.
- [`move_file`](tools/move_file.md) — JS/TS relocation review plan.

## Advisory gates

- [`move_symbol`](tools/move_symbol.md) — projected move risk; no source edit.
- [`delete_readiness`](tools/delete_readiness.md) — review-required deletion verdict; no deletion.
