# `share_method_body`

Use to plan one helper for identical method bodies on different types. It does not write. Apply the returned edit plan with `apply_edit_plan`.

## Boundary

Both methods stay and become wrappers. The helper takes the fields the body reads, in the order they appear. Same-type copies stay with `remove_duplicates`. A symbol that is not a method, or whose declaration end is not proven, returns `NOT_A_SYMBOL`. More than 200 methods returns `BLOCKED` and plans nothing. Completeness is `PARTIAL` and warns `FIELD_READS_ONLY`.

Pass `symbol` to limit the plan to that method's body. Omit it to scan the repository.

## Inputs

- `symbol` (string, optional) — exact symbol id.
- `output_format` ("text" | "json", default "text").

## Minimal call

```json
{
  "name": "share_method_body",
  "arguments": {}
}
```

Preview and inspect `helpers` and `kept` before any apply. The live MCP `tools/list` schema remains authoritative for this installed version.
