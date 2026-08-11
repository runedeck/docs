## Why

RuneSpec and OpenSpec offer similar specification workflows through different command interfaces. Routing one interface through the other would couple unrelated command grammars, and maintaining separate writable trees would create synchronization and review problems. One canonical artifact tree lets both CLIs operate on the same files with no conversion or mirror.

## What Changes

- Fix the canonical repository layout: `docs/openspec/` is the only writable lifecycle tree, `docs/.openspec-store/store.yaml` gives the store a stable registry identity, and repository configuration carries `spec.root: docs/openspec`.
- Pin OpenSpec v1.7.0 as the compatibility baseline for shared artifacts.
- Keep both CLIs independent: `rune spec` always executes RuneSpec, `openspec` always executes OpenSpec, and neither invokes the other.
- Confine Rune-specific runtime state to Rune-namespaced, gitignored paths inside the shared tree.

## Capabilities

### New Capabilities

- `spec-compatibility`: Defines the canonical store layout, the compatibility baseline, and the conditions a Rune-specific extension must satisfy to enter the shared tree.

### Modified Capabilities

None.

## Impact

- Repositories adopting the layout move existing lifecycle directories into `docs/openspec/` without content transformation.
- Both validators run against the same tree during rollout; a repository is compatible only when both pass.
- RuneSpec keeps every current command, flag, output form, root mode, transaction path, and public API.
