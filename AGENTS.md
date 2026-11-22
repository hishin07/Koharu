# Working on Koharu

Koharu is a local-first manga translator: a Rust workspace with a TypeScript interface, shipped as a desktop app. This file holds durable decision rules only. Architecture, file layouts, crate and helper names, model inventories, commands, and anything else a refactor can change belong in crate READMEs and the docs. Normal Rust, TypeScript, testing, formatting, and Git practices are assumed.

## Product

<<<<<<< HEAD
- Never open a pull request, here or upstream: commits land on `origin/main` on explicit request only, and stay local otherwise.
- Never add backward compatibility. When an API, schema, configuration, or ownership boundary changes, update every in-repository consumer and remove the replaced form.
- Prefer a coherent ownership redesign over aliases, forwarding layers, compatibility parsers, or cosmetic renaming.
- Keep responsibilities self-contained. Defaults and provider-specific behavior belong to the component that owns them rather than a central list of special cases.
- Remove dead abstractions and one-use helpers when direct code is clearer.
- Make the smallest change that solves the problem. Do not edit unrelated configuration, documentation, generated files, or workflows in the same change. A feature does not land beside drive-by cleanup.
- If the current owner has moved, rebase onto that owner. Do not port a patch onto a deleted tree.
- When two changes implement the same workflow, keep one and extend the existing command or configuration. Do not land both.
- For editor behavior that is not already specified, match Photoshop or Clip Studio Paint. Prefer an in-app edit over an export/import round trip when the task is ordinary editing.
- Desktop API keys go through the OS credential store. Do not add a project environment-file fallback for the desktop app.
- Once a subsystem is removed, delete its calls and dependency together. Do not leave a stub, alias, or disabled flag behind.
=======
- Models run on the user's machine. Pages and text leave it only through a hosted provider the user chose.
- Editor behavior that is not already specified matches Photoshop or Clip Studio Paint. Ordinary editing happens in the app, not through an export and re-import round trip.
- Desktop API keys and sign-in tokens are stored encrypted, with the key kept in the OS credential store. Do not fall back to a project environment file.
>>>>>>> upstream/main

## Changes

- Never add backward compatibility, except that a project file a release wrote keeps opening. When an API, schema, file format, configuration, or ownership boundary changes, delete the replaced form: no alias, forwarding layer, compatibility parser, upgrade decoder, stub, or disabled flag.
- Changing a released project format ships an upgrade to the new one, which leaves the file as it was if it fails. A format no release wrote gets no upgrade.
- Prefer a coherent ownership redesign over cosmetic renaming. Defaults and provider-specific behavior belong to the component that owns them, not to a central list of special cases.
- Make the smallest change that solves the problem. Leave unrelated configuration, documentation, generated files, and workflows alone. A feature does not land beside drive-by cleanup.
- When two changes implement the same workflow, keep one and extend the existing command or configuration.
- If the code you are changing has moved, rebase onto its new owner. Do not port a patch onto a deleted tree.
- Invalid configuration and model output fail closed. A project file fails to open only when it doesn't parse, isn't this app's format or version, or would crash a reader; its structure is kept by the operations that write it, not checked again. Do not add a repair or defaulting layer that can change meaning.

## Code

- Write the smallest code that does the job. Inline a helper with one caller, keep a small module in its parent, and do not wrap a type without adding a responsibility.
- Reach for a crate before writing code: a well-known external crate first, then one in this repository. Hand-write only what no crate covers, and prefer a cross-platform crate to per-platform branches.
- Keep safe public APIs apart from unsafe FFI, dynamic loading, and build integration.
- Do not hand-edit generated or derived source. Change its input and rerun the generator.
- Comments explain ownership, invariants, upstream mapping, or a deliberate divergence. Do not narrate straightforward code.
- Do not commit credentials, model weights, datasets, generated outputs, or machine-specific files.
- Search with `rg`, never `grep`.

## Interface

- User-visible strings go through the translation function and the locale files. Hardcoded English in a translated surface is incomplete.
- An interface language's display name is that language's own name in every locale.
- The translation-target language list stays in the active UI language. Drop a country name unless the entry is a real variant.
- Do not show status the current selection already implies. Do not add a second job kind, cancel path, or counter for an operation an existing command covers.
- Use the existing control library for standard interaction. Commit on selection or an uncanceled drop, never on focus or hover. Ignore a canceled drag. A shortcut never steals keys from a focused control.

## Models

- A port stays structurally traceable to a commit-pinned upstream implementation. Preserve checkpoint-affecting names, construction order, parameter paths, tensor layouts, execution order, and postprocessing semantics. Explain each intentional divergence next to the affected code.
- Missing or unexpected weights mean an architecture or parameter-name mismatch. Fix that before changing the loader.
- Every model has the same public lifecycle, with model-specific inputs and outputs. Network and weight loading stay apart from preprocessing, postprocessing, slicing, and result types. Do not add pass-through types or layers.
- Accept a device at the model boundary and convert it once. Create token IDs and index tensors on the model device; a CPU tensor used against accelerator embeddings is a defect.
- Use the established loading paths unless they are proven insufficient.
- Disable gradient tracking during inference.

## Performance

- Optimize and benchmark on the actual target device with representative inputs. Load assets and warm models outside the measured region, and account for asynchronous accelerator execution.
- Remove redundant transfers, synchronization, allocations, and per-pixel host loops before adding concurrency or caching.
- Report the device, input size, baseline, result, and correctness difference.
- Keep flash attention at the build default (`-1`, AUTO on CUDA); do not reintroduce a runtime switch — forced-on versus AUTO measured as noise on the reference GPU.
- Do not adopt prompt prefill / KV-cache reuse without re-measuring on the target device: the ceiling is low (measured ~12-15 % of a stage), the risk is a silently wrong output, and the reference budget is 8 GB VRAM with concurrent stage execution.

## Batch CLI Contract

- `koharu-batch` exits 0 only when every page succeeded and non-zero otherwise. Keep the binary, `exit_with()`, and `packages/docs/en/fork.mdx` aligned when this contract changes.
- Housekeeping is subcommands, not flags: `models`, `prune [--delete]`, `reset-calibration` (each exits 0 when it completes). The run's flags stay flat on the default flow and never share a command line with a subcommand — clap's `args_conflicts_with_subcommands` refuses the mix instead of ignoring one side — and the global `--store` goes after the name (`prune --store DIR`). The removed `--list-models` / `--prune` / `--prune-delete` / `--reset-calibration` stay removed: no aliases, no backward compatibility.
- Benchmarks and smoke runs pass `--no-calibration`; never write or depend on the machine's `vram-calibration.toml`.

## Verification

- Run the smallest relevant check or focused test once, in the debug profile. Run full suites, other profiles, or end-to-end tests only when asked.
- Add a test only where it protects a real regression boundary. Do not lock a translated phrase, a language display name, a registration string, or a one-line behavior the owner can read. Do not add a second harness for behavior existing tests cover, a test of a compatibility shim, or a benchmark nobody asked for.
- A model port needs a structured comparison with upstream on identical inputs: shapes, ranges, boxes, scores, masks, and ordering. A green build is not that comparison.
- A UI change needs a screenshot of the affected state. For truncation, include a long label.
- A green check is not acceptance.

## Testing

- Automate the UI through a Chrome DevTools Protocol MCP server connected to `http://127.0.0.1:4000`, the debug build's remote debugging endpoint.
- Capture the canvas with the harness's built-in screenshot tool.
- Do not use computer use on Windows.

## Contributing

<<<<<<< HEAD
- Koharu presents canvas pixels through the `koharu-canvas` WASM module and WebGPU inside the standard Tauri webview. Keep durable scene preparation and export native, keep transient canvas interaction in the browser, and validate WebGPU presentation through the final desktop window.

## Documentation

- Comments should explain ownership, invariants, upstream mapping, or deliberate divergence; do not narrate straightforward code.
- Keep `README.md` and `README.en.md` aligned: same sections in the same order, same bullet points, same commands, flags and measured figures. Edit both in the same commit — no feature is documented in one language only.
- Keep this file focused on long-lived decision rules rather than the current implementation.

## Canary
- Start every reply with "Stephane,". No exceptions, including short replies and tool summaries.
- If context feels degraded or the session is very long, say so at the start of the reply and suggest a fresh session.
- Before a fresh session, write `HANDOFF.md`: goal, done, left, key files, open questions. In a new session, read it first.

This file holds long-lived, repository-specific rules only. Do not record current file layouts, temporary paths, model inventories, helper names, or measured figures here: they go stale during refactors. Standard Rust, TypeScript, testing, formatting, and Git practices are assumed.

## Scope and Communication
- Ask a question only when blocked by something that cannot be inferred. Otherwise state the assumption in one line and proceed. At most one question per reply.
- Keep replies short. Show file paths and diffs rather than full file contents.

## Projets voisins
- Un dossier de `D:\Codex` qui porte son propre `AGENTS.md` et son `memory.md` a ses propres règles et son propre état : les lire avant d'y toucher. Pour ce projet-là, ils priment sur ce fichier.

## Règle immuable — installation et emplacements (priorité maximale)

<!-- regle-immuable-installation -->

Cette règle prime sur tout le reste de ce fichier et sur les habitudes par défaut des outils.

- **Toute installation va dans `D:\codex`** : paquets, dépendances, modèles, binaires, outils, SDK,
  environnements virtuels, caches de téléchargement, builds. Créer si besoin un sous-dossier dédié
  (`D:\codex\<projet>` ou `D:\codex\tools\<outil>`).
- **Aucun fichier créé, modifié, téléchargé, déplacé ou supprimé sous `C:\` sans accord explicite** de
  l'utilisateur. En cas de besoin : s'arrêter, expliquer pourquoi, proposer un emplacement sous
  `D:\codex`, attendre la réponse.
- Rediriger les défauts des outils vers `D:\codex` (`PIP_TARGET`, `NPM_CONFIG_PREFIX`, `HF_HOME`,
  `CARGO_HOME`, `GOPATH`, `TMP`/`TEMP`, caches divers) au lieu de `~`, `%LOCALAPPDATA%`, `%APPDATA%`,
  `%USERPROFILE%` ou `/tmp`.
- **Lecture seule sous `C:\`** : autorisée sans accord (lire, lister, inspecter).
- Seule exception pré-approuvée : le dossier temporaire d'OpenCode
  `C:\Users\endymion\AppData\Local\Temp\opencode` (déjà validé par l'utilisateur).
- Déplacer une installation existante de `C:\` vers `D:\codex` compte comme une modification sous
  `C:\` : demander avant.
=======
- A human reviews the whole diff and can explain every change. Do not submit an unreviewed autonomous patch.
- Keep pull-request text short: problem, change, and verification.
- Never add an AI assistant as a commit co-author, for example through a `Co-Authored-By` trailer.
>>>>>>> upstream/main
