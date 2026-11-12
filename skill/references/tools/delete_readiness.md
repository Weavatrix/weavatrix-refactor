# `delete_readiness`

Use to obtain a review-required deletion verdict and exact span; it never deletes source.

## Boundary

A private symbol with no incoming use can be `safe: true`. The verdict stays `REVIEW_REQUIRED` and nothing is written. A public or exported symbol stays not safe. Confirm external and dynamic use before deleting.

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
