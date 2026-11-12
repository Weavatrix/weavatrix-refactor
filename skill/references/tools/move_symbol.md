# `move_symbol`

Use to review cycles and architecture risk and to plan cutting the declaration into the destination file.

## Boundary

The status stays `EVALUATED` and the fidelity is `PROJECTED_FROM_GRAPH_EDGES`. The plan moves the declaration, retargets the imports it can prove, and copies the imports the declaration uses. Apply that plan with `apply_edit_plan`.

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
