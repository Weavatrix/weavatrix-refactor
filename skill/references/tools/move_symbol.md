# `move_symbol`

Use to review cycles and architecture risk and to plan cutting the declaration into the destination file.

## Boundary

The status stays `EVALUATED` and the fidelity is `PROJECTED_FROM_GRAPH_EDGES`. The result also includes an edit plan. Apply that plan with `apply_edit_plan`. A sole `use ...::name;` import of the symbol is removed with the declaration.

## Inputs

- `symbol` (string, required).
- `to_file` (string, required).
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "move_symbol",
  "arguments": {
    "symbol": "<exact symbol id>",
    "to_file": "<destination file>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
