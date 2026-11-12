# `bulk_replace`

Use for an occurrence-selective literal or regex replacement after reviewing stable occurrence ids or an expected count.

## Boundary

The first call should discover occurrences; the second selects ids or supplies `expected_count`. Matches include comments and strings. This version has no trivia filter.

## Inputs

- `pattern` (string, required).
- `replacement` (string, required).
- `expected_count` (integer).
- `flags` (string).
- `literal` (boolean, default true).
- `occurrence_ids` (array<string>).
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.
- `path_prefix` (string).

## Minimal call

```json
{
  "name": "bulk_replace",
  "arguments": {
    "pattern": "<pattern>",
    "replacement": "<replacement>",
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
