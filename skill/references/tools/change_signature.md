# `change_signature`

Use for JavaScript or TypeScript parameter addition/removal with graph-derived call-site surgery; always review the partial result.

## Boundary

This operation is JavaScript/TypeScript-only and always `PARTIAL`; spread and value-add sites remain uncertain.

## Inputs

- `operation` (object, required).
- `symbol` (string, required).
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "change_signature",
  "arguments": {
    "operation": {},
    "symbol": "<exact symbol id>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
