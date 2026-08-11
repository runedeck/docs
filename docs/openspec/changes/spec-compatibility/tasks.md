## 1. This repository

- [x] 1.1 Move the lifecycle tree to `docs/openspec/` without content transformation
- [x] 1.2 Add `docs/.openspec-store/store.yaml` with the stable store id
- [x] 1.3 Set `spec.root: docs/openspec` in repository configuration
- [x] 1.4 Run RuneSpec validation against the moved tree
- [x] 1.5 Run the pinned OpenSpec full validation against the same tree
- [ ] 1.6 Register `docs/` in the machine-local OpenSpec store registry
- [ ] 1.7 Exercise the interactive OpenSpec view against the registered store

## 2. Ecosystem rollout

- [ ] 2.1 Add the interrupted-operation fixture proving OpenSpec ignores Rune runtime state
- [ ] 2.2 Adopt the layout in each participating repository through its own reviewed pull request
- [ ] 2.3 Record per-repository store ids and confirm no registry collisions
