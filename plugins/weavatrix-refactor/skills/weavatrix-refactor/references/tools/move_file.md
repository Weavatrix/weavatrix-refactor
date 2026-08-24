# `move_file`

Use for a JavaScript or TypeScript file-relocation review plan with importer rewrites and architecture dry-run.

## Boundary

This returns a review plan that the agent applies mechanically; it is not an automatic apply envelope.

## Inputs

- `from` (string, required).
- `to` (string, required).
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "move_file",
  "arguments": {
    "from": "<source file>",
    "to": "<destination file>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
