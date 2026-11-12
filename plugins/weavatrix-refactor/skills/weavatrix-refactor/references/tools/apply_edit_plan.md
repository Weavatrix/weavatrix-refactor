# `apply_edit_plan`

Use to verify and apply a compatible `weavatrix.edit-plan.v1` envelope produced by another planning method.

## Boundary

`mode:"apply"` requires the write gate and the token returned by previewing the same exact plan. A rename entry missing `from`, `to`, or `sha256`, or a delete entry missing `path` or `sha256`, refuses the whole plan as `INVALID_PLAN`. No other entry in that plan is applied.

## Inputs

- `plan` (object, required) — The weavatrix.edit-plan.v1 envelope, verbatim as produced by the planning tool.
- `confirm_token` (string) — Required for mode="apply"; issued by the preview step, single-use, 5-minute TTL.
- `mode` ("preview" | "apply", default "preview").
- `output_format` ("text" | "json", default "text") — text returns concise TextContent; json also returns the stable structuredContent envelope.

## Minimal call

```json
{
  "name": "apply_edit_plan",
  "arguments": {
    "plan": {},
    "output_format": "text"
  }
}
```

Preview and inspect evidence before any apply. The live MCP `tools/list`
schema remains authoritative for this installed version.
