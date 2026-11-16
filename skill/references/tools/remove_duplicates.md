# `remove_duplicates`

Use to plan deletion of exact duplicate function or method declarations. It does not write. Apply the returned edit plan with `apply_edit_plan`.

## Boundary

Two declarations match only when their source, including the name, is the same bytes. One copy stays: the earliest unused copy when nothing else anchors the family, otherwise every copy that is exported, still used, or able to load code dynamically. A nested declaration is not planned on its own. More than 200 deletions returns `BLOCKED` and plans nothing. A symbol that is not a function or method returns `NOT_A_SYMBOL`. Completeness is `PARTIAL`: uses are the ones the graph recorded. The file is parsed after the deletion. A result that does not parse is left in `kept`.

Pass `symbol` to limit the plan to that declaration's copies. Omit it to scan the repository.

## Inputs

- `symbol` (string, optional) — exact symbol id.
- `output_format` ("text" | "json", default "text").

## Minimal call

```json
{
  "name": "remove_duplicates",
  "arguments": {}
}
```

Preview and inspect `deleted` and `kept` before any apply. The live MCP `tools/list` schema remains authoritative for this installed version.
