# `bulk_replace`

Use for an occurrence-selective literal or regex replacement after reviewing stable occurrence ids or an expected count.

## Boundary

The first call should discover occurrences; the second selects ids or supplies `expected_count`. Matches are indexed code. Comments and strings are included only when `include_trivia` is true. An extension the tokenizer does not know still skips `//` comments, block comments, quotes, and a `#` that starts a line.

## Inputs

- `pattern` (string, required).
- `replacement` (string, required).
- `expected_count` (integer).
- `flags` (string).
- `literal` (boolean, default true).
- `include_trivia` (boolean, default false) — comments and strings count only when this is true.
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
