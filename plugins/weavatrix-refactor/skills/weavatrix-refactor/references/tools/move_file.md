# `move_file`

Use to plan a filesystem rename together with the relative import specifiers that point at the file.

## Boundary

The plan includes the rename and the specifier rewrites. Apply it with `apply_edit_plan`. A missing parent directory is created at apply and left in place on rollback. This call does not write by itself.

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
