# `rollback_last_apply`

Use immediately after an applied transaction must be reverted and the touched files have not drifted.

## Boundary

Rollback restores the newest retained apply and refuses to overwrite post-apply drift. An empty directory created for a rename is removed. A later edit of the same path invalidates the older receipt, so only the newest apply of that path can be rolled back.

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
