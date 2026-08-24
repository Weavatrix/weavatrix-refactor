---
name: weavatrix-refactor
description: >-
  Use Weavatrix Refactor only when the user explicitly requests repository
  source changes that benefit from a proven refactor workflow: symbol rename,
  coordinated rename, signature change, symbol edit, selected replacement,
  import cleanup, file or symbol move planning, deletion readiness, exact plan
  application, or rollback. Skip it for analysis-only tasks and ordinary small
  edits that do not need a refactor plan.
---

# Weavatrix Refactor

Use this extension on demand after the target symbol, file, or change is clear.
The plugin exposes only 11 refactor methods; use the separate `weavatrix`
skill for the 43 read-only repository-intelligence methods.

## Workflow

1. Identify an exact symbol or bounded target. Use Weavatrix core first when
   cross-file graph evidence is needed.
2. Call the narrowest Refactor method in its default preview/planning mode.
3. Review exact files, hashes, edits, completeness, warnings, and
   `uncertainReferences`.
4. Apply only when the user requested the source change and the plan matches
   that request. Repeat the identical operation with `mode:"apply"` and its
   single-use `confirm_token`.
5. Run repository-native formatting and tests. Use `rollback_last_apply` if
   the applied transaction must be reverted before files drift.

Read [references/tool-routing.md](references/tool-routing.md) to choose a
method, then open only that method's card.

## Safety

- Installing the plugin opens the process write gate, but no edit can apply
  without a matching preview token.
- `PARTIAL`, `UNPROVEN`, uncertain references, and advisory move results are
  review states, not permission to guess.
- Never invent a symbol id, silently broaden a selected occurrence set, or
  convert an advisory plan into an automatic source write.
- Rollback is all-or-nothing and refuses post-apply drift.
