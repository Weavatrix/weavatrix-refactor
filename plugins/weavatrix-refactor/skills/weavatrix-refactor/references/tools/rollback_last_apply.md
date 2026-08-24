# `rollback_last_apply`

Use immediately after an applied transaction must be reverted and the touched files have not drifted.

## Boundary

Rollback is all-or-nothing and refuses to overwrite post-apply drift.

## Inputs

- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "rollback_last_apply",
  "arguments": {
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
