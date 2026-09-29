# MEROA third-party notices

MEROA is distributed as a Tauri application. Direct runtime and build
dependencies are pinned by `package-lock.json` and `src-tauri/Cargo.lock`.
Their upstream license and notice files remain the authoritative source for
the complete dependency tree.

The direct packages used by the application include Tauri and its autostart/
single-instance plugins, React, Vite, and TypeScript. Their published package
metadata identifies the applicable MIT and/or Apache-2.0 licensing terms.
The Rust dependency graph is described by `src-tauri/Cargo.toml` and
`src-tauri/Cargo.lock`; no dependency license text is rewritten here.

Font licensing is documented separately in [FONT_LICENSES.md](FONT_LICENSES.md),
with the complete SIL Open Font License texts preserved beside the distributed
font files.

For a public redistribution, verify the current transitive dependency notices
against the exact lockfiles used for that build.
