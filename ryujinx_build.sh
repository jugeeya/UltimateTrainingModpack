set -eu

# Obviously adjust these based on your paths
RYUJINX_APPLICATION_PATH="/mnt/c/Users/Jdsam/Downloads/ryujinx-1.1.119-win_x64/publish/Ryujinx.exe"
SMASH_APPLICATION_PATH="C:\Users\Jdsam\Downloads\Super Smash Bros. Ultimate (World) (En,Ja,Fr,De,Es,It,Nl,Zh-Hant,Zh-Hans,Ko,Ru)\Super Smash Bros. Ultimate (World) (En,Ja,Fr,De,Es,It,Nl,Zh-Hant,Zh-Hans,Ko,Ru).xci"
RYUJINX_SMASH_SKYLINE_PLUGINS_PATH="/mnt/c/Users/Jdsam/AppData/Roaming/Ryujinx/mods/contents/01006a800016e000/romfs/skyline/plugins"

# Build with release feature
cargo skyline build --release --features layout_arc_from_file

# Copy over to plugins path
cp target/aarch64-skyline-switch/release/libtraining_modpack.nro $RYUJINX_SMASH_SKYLINE_PLUGINS_PATH

# Run Ryujinx
$RYUJINX_APPLICATION_PATH "${SMASH_APPLICATION_PATH}"

# Here, you can run `cargo skyline set-ip {IP address...}; cargo skyline listen` for logs