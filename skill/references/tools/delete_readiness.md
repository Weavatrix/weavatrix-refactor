# `delete_readiness`

Use to obtain a review-required deletion verdict and exact span; it never deletes source.

## Boundary

Exported symbols cannot receive a proven-safe verdict; confirm external and dynamic use before deleting.

## Inputs

- `symbol` (string, required).
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "delete_readiness",
  "arguments": {
    "symbol": "<exact symbol id>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
