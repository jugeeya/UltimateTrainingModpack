# Build the layout archive

Edit the BFLYT JSON files in `bflyt/` or the PNG files in `bntx/textures/`.
The PNGs are the source of truth for textures; SVG copies are not used.
If you add or rename a file, update `patch.json` or `bntx/bntx-patch.json`.
Then build the plugin as usual:

```sh
cargo-skyline skyline build --release
```

`build.rs` uses the pinned `smush-ui` Rust library to patch
`src/static/layout_original.arc` before compilation. Cargo reruns the patch
when the base archive, a JSON file, or a PNG changes. `cargo check --locked`
also generates the archive, so you do not need Python or a separate command.
Git ignores the generated `src/static/layout.arc`.

The patch preserves BFLAN entries from the original archive. It discards the
unintentional BFLAN changes in the old generated archive. BC1 texture encoding
is lossy, so a few texture previews differ from that archive.
