# `move_symbol`

Use only to evaluate blast radius, cycles, and architecture risk before a manual symbol move.

## Boundary

This is projected graph evidence only and produces no byte-exact edits.

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
