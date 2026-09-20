# Tasks

## 1. Layout

- [x] 1.1 Move the tree to `docs/`, drop `docs/openspec/config.yaml` and the `spec.root` override
- [x] 1.2 Rename the five two-word ids and split the three long requirements
- [x] 1.3 Archive `openspec-store-compatibility` as abandoned

## 2. Record

- [x] 2.1 `module.yaml` and `defaults.yaml` so `rune adr adopt` opens a review session here
- [x] 2.2 Adopt CORE-0019 as `DOCS-0001`, 12 blocks reviewed, sealed
- [ ] 2.3 Archive moves this change and links `DOCS-0001` as its record

## 3. Verification

- [x] 3.1 `rune spec doctor`, `rune spec validate`, `rune docs check`, `rune adopt doctor`: no error
- [ ] 3.2 Both prek stages in an isolated clone
