# `rename_related_symbols`

Use for a coordinated rename set when names may conflict, chain, or swap. The operation uses the same cross-language sites as `rename_symbol`.

## Boundary

Several renames land in one transaction. Chains and swaps apply together. A site the graph does not prove stays uncertain, and the preview stays `PARTIAL` when any of those remain. The catalog text still says JavaScript and TypeScript; the operation does not refuse another indexed language.

## Inputs

- `renames` (array<object>, required, min items 1, max items 50).
- `confirm_token` (string, max length 128) — Required for mode="apply"; single-use and bound to the exact generated plan and working tree.
- `mode` ("preview" | "apply", default "preview") — preview verifies the generated plan and returns a confirm_token; apply recomputes the same plan and consumes that token.
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "rename_related_symbols",
  "arguments": {
    "renames": [{}],
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
