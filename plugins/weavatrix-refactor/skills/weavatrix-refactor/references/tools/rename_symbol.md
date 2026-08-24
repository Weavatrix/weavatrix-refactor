# `rename_symbol`

Use for one cross-file symbol rename. Preview first; apply only by repeating the identical symbol and new name with the returned token.

## Boundary

Ambiguous names return candidate ids. Never invent an id or apply a partial plan without reviewing `uncertainReferences`.

## Inputs

- `new_name` (string, required).
- `symbol` (string, required) — Exact symbol id (file#name@line).
- `confirm_token` (string, max length 128) — Required for mode="apply"; single-use and bound to the exact generated plan and working tree.
- `mode` ("preview" | "apply", default "preview") — preview verifies the generated plan and returns a confirm_token; apply recomputes the same plan and consumes that token.
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "rename_symbol",
  "arguments": {
    "new_name": "<new name>",
    "symbol": "<exact symbol id>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
