# TabletCraft / VectorCraft — Codex Handoff

## Current state

Repository: `TabletCraft/vectorcraft`

Working branch: `tablet/poc`

Do not work directly on `main`. `main` should remain close to Storytold upstream so future upstream synchronization stays manageable.

This fork is the first implementation proving ground for **TabletCraft**, an open-source suite of separate tablet-native creative applications based on Storytold's Crafting Apps ecosystem. The first three intended apps are Vector, Raster/Photo, and CAD, but **VectorCraft is the only implementation target right now**.

Target platforms are iPadOS and Android tablets. The initial POC is iPad-first, with Apple Pencil input. The intended architecture is a shared Rust application/core layer with platform-native UI shells rather than porting the existing desktop egui UI directly.

## Architecture findings

VectorCraft already has a useful separation between frontend and core behavior:

- `vectorcraft-engine` owns `Session`, document/session state, history, selection, command execution, etc.
- `vectorcraft-tools` accepts pointer/tool input and produces actions/overlays rather than directly mutating the document.
- `vectorcraft-render` currently uses the CPU renderer (`vello_cpu`) and produces premultiplied RGBA pixel buffers.
- `vectorcraft-ui-egui` is the existing desktop frontend. It owns desktop input interpretation, screen/document coordinate conversion, canvas navigation, texture upload, and presentation.

The existing tool input path already supports a `PointerEvent` containing pointer kind, document position, modifiers, and normalized pressure. Do not replace this architecture for the tablet port.

The initial tablet rendering plan is **not** to rewrite VectorCraft rendering in Metal/Vulkan. For the POC, use the existing VectorCraft CPU renderer and pass/upload its RGBA framebuffer to the native platform presentation layer. GPU-native rendering can be evaluated later only if profiling justifies it.

Expected POC flow:

```text
Apple Pencil
    -> UIKit native event
    -> tablet/native bridge
    -> VectorCraft PointerEvent
    -> vectorcraft-engine::Session
    -> vectorcraft-tools
    -> engine commands/document
    -> vectorcraft-render CPU rasterizer
    -> RGBA framebuffer
    -> native iPad presentation
```

## Tablet-specific code added so far

A new workspace crate has been started at:

`crates/tablet`

Package name:

`vectorcraft-tablet`

The root workspace already includes `crates/*`, so the crate is automatically a workspace member. A `vectorcraft-tablet = { path = "crates/tablet" }` workspace dependency was also added.

Initial files:

```text
crates/tablet/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── input.rs
    ├── view.rs
    ├── render.rs
    └── session.rs
```

The intended public façade currently consists of:

- `TabletSession` — native-facing owner/facade around VectorCraft session/render state.
- `TabletView` — platform-independent center/zoom/rotation state and document-to-pixel transform.
- `TabletPointer` — minimal native pointer sample (`x`, `y`, normalized `pressure`).
- `TabletFrame` — native-facing RGBA framebuffer (`width`, `height`, `pixels`).

This API is intentionally minimal. Do not add tilt, azimuth, altitude, eraser state, barrel buttons, predicted/coalesced event structures, or a large generic input framework until actual iPad Pencil integration demonstrates what is needed.

## Immediate next task

**Compile-check the `tablet/poc` branch before adding more architecture.**

Start with something equivalent to:

```bash
git checkout tablet/poc
git pull
cargo check -p vectorcraft-tablet
```

Fix compile errors in the new tablet crate while preserving the existing engine/tool/render architecture. Do not perform unrelated cleanup or broad refactors.

After `vectorcraft-tablet` compiles cleanly, implement the first real vertical slice:

1. Create/own a usable VectorCraft `Session` from `TabletSession`.
2. Create a blank VectorCraft document using existing engine/document APIs rather than duplicating document initialization.
3. Render the active document with `vectorcraft-render` using `TabletView::render_transform`.
4. Return the rendered pixels through `TabletFrame`.
5. Add focused tests where practical.

Only after that works should the tablet crate begin translating native pointer samples into existing `vectorcraft_tools::PointerEvent` calls through `Session::pointer()`.

## POC success criterion

The first major milestone is deliberately narrow:

```text
native iPad app
    -> blank VectorCraft document
    -> Apple Pencil Down/Move/Up + pressure
    -> Rust tablet façade
    -> existing VectorCraft tool/engine pipeline
    -> document mutation
    -> existing renderer
    -> RGBA framebuffer
    -> visible stroke on iPad
```

No production toolbar, panels, layer UI, Android shell, file browser, or custom GPU renderer is required for this milestone.

## Constraints

- Preserve Storytold's existing architecture and repository guidance in `AGENTS.md`.
- Keep tablet/platform concerns out of lower-level crates unless a genuinely platform-neutral upstream improvement is required.
- Prefer tablet-specific additions in `crates/tablet` and, later, native app directories such as `apps/vectorcraft-ipad` and `apps/vectorcraft-android`.
- Avoid gratuitous changes to upstream files. Keeping the downstream diff small is a project goal.
- Generic improvements that benefit upstream VectorCraft should be kept separable so they can potentially be contributed back to Storytold.
- Do not port the desktop egui UI wholesale. Native tablet UI should be designed for touch/stylus interaction.
- Do not introduce a shared cross-app `tablet-core` repository yet. Extract shared infrastructure only after VectorCraft proves a real reusable requirement.
- Preserve existing license/copyright notices. TabletCraft must not use protected ArtCraft branding/wordmarks/logos in its modified distribution.

## Longer-term tablet direction

After the iPad POC proves the boundary, the intended architecture is roughly:

```text
Storytold VectorCraft core crates
        ^
        |
vectorcraft-tablet Rust façade
       / \
      /   \
 iPadOS   Android
Swift/UI  Kotlin/Compose
UIKit     native stylus APIs
Metal     native surface/presentation
```

The same architectural pattern may later be applied to Storytold PhotoCraft and CADCraft, but do not generalize prematurely.

## Working rule for Codex

Before making architectural changes, inspect the actual VectorCraft APIs and use them rather than inventing parallel implementations. Keep each change small, compiling, and easy to compare against Storytold upstream.
