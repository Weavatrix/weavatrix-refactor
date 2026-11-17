# `extract_window`

Use to plan a helper for a repeated straight-line statement window inside Rust functions. It does not write. Apply the returned edit plan with `apply_edit_plan`.

## Boundary

A window is a run of statements of at least four lines that is not the whole function. The copies must be the same bytes. Every free name needs a written type. A copy under a different `cfg` attribute, a copy that returns, and a copy that reads `self` stay in `kept`. More than 200 windows returns `BLOCKED` and plans nothing. A symbol that is not a function or method returns `NOT_A_SYMBOL`. Completeness is `PARTIAL` and warns `EXPLICIT_TYPES_ONLY`.

Pass `symbol` to limit the plan to windows inside that function or method. Omit it to scan the repository.

## Inputs

- `symbol` (string, optional) — exact symbol id.
- `output_format` ("text" | "json", default "text").

## Minimal call

```json
{
  "name": "extract_window",
  "arguments": {}
}
```

Preview and inspect `extracted` and `kept` before any apply. The live MCP `tools/list` schema remains authoritative for this installed version.
