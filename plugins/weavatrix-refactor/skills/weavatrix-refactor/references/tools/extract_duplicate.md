# `extract_duplicate`

Use to plan extraction of an exact duplicate function that is still used. It does not write. Apply the returned edit plan with `apply_edit_plan`.

## Boundary

Two functions match only when their declaration source, including the name, is the same bytes. The earliest non-exported Rust copy stays. The later copies are deleted and the calls the graph proves are retargeted to the copy that stays. A family whose copies are all unused stays with `remove_duplicates`. Identical methods stay with `share_method_body` and return `NOT_A_SYMBOL` when named here. A declaration whose end is not proven returns `NOT_A_SYMBOL`. More than 200 deletions returns `BLOCKED` and plans nothing. Completeness is `PARTIAL` and warns `GRAPH_PROVEN_USES_ONLY`: retargeted calls are the ones the graph recorded.

Pass `symbol` to limit the plan to that function's copies. Omit it to scan the repository.

## Inputs

- `symbol` (string, optional) — exact symbol id.
- `output_format` ("text" | "json", default "text").

## Minimal call

```json
{
  "name": "extract_duplicate",
  "arguments": {}
}
```

Preview and inspect `deleted` and `kept` before any apply. The live MCP `tools/list` schema remains authoritative for this installed version.
