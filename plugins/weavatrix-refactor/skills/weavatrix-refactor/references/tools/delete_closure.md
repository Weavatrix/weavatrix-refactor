# `delete_closure`

Use to plan deletion of one symbol or endpoint together with the callees that nothing outside that closure uses. It does not write. Apply the returned edit plan with `apply_edit_plan`.

## Boundary

A file, module, package, or repository returns `NOT_SUPPORTED`. A symbol with no source span returns `NOT_SUPPORTED`. A handler counts when its name is a whole word on the same route line in the same file. `scope` defaults to `exclusive`. That mode returns `BLOCKED` when another contract still uses the selection, and it lists shared helpers in `kept` instead of deleting them. Exclusive mode will not delete a repository. An endpoint whose handler is shared still plans removal of that route only. A function also removes the one route that passes it as a handler; two such routes block, so the other endpoint stays. `scope:"connected"` also deletes the selection's direct referrers, and for an endpoint the shared handler plus that handler's direct callers. Callers of those referrers are listed in `affected` and are not walked further. Completeness is `PARTIAL`: the plan proves the uses the graph recorded, plus a handler named on the route line when the graph has no edge for that argument.

Select an endpoint to remove that route and a handler no other function calls. Select a function when the function itself is the thing to remove. `delete_readiness` stays a verdict and never plans a deletion.

## Inputs

- `symbol` (string, required) — exact symbol or endpoint id.
- `scope` ("exclusive" | "connected", default "exclusive").
- `output_format` ("text" | "json", default "text").

## Minimal call

```json
{
  "name": "delete_closure",
  "arguments": {
    "symbol": "<exact symbol id>",
    "scope": "exclusive"
  }
}
```

Preview and inspect `deleted`, `kept`, and `affected` before any apply. The live MCP `tools/list` schema remains authoritative for this installed version.
