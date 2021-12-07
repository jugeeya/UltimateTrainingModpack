# Clone the repository
git clone --recursive

# Build the training modpack Skyline Plugin
# The resulting build is found in target/aarch64-skyline-switch/release/libtraining_modpack.nro
cargo skyline build --release

# Make directories
rm -r release
mkdir -p release
mkdir -p release/atmosphere/contents/01006A800016E000
mkdir -p release/atmosphere/contents/01006A800016E000/romfs/skyline/plugins
mkdir -p release/atmosphere/contents/01006A800016E000/manual_html/html-document/contents.htdocs

# Download additional files
## Skyline
wget https://github.com/skyline-dev/skyline/releases/download/beta/skyline.zip
unzip skyline.zip
rm skyline.zip
## Params-hook plugin
wget https://github.com/ultimate-research/params-hook-plugin/releases/download/v0.1.1/libparam_hook.nro
## NRO hook plugin
wget https://github.com/ultimate-research/nro-hook-plugin/releases/download/v0.1.1/libnro_hook.nro
## NN HID hook plugin
wget https://github.com/jugeeya/nn-hid-hook/releases/download/beta/libnn_hid_hook.nro
## Smash visualizer plugin
wget https://github.com/blu-dev/smash-visualizer/releases/download/0.1.0/Smash-Visualizer-0.1.0.zip
unzip -o Smash-Visualizer-0.1.0.zip
rm Smash-Visualizer-0.1.0.zip

# Move files to release
mv atmosphere/contents/01006A800016E000/romfs/skyline/plugins release/atmosphere/contents/01006A800016E000/romfs/skyline
rm -r atmosphere
rm -r exefs
mv libparam_hook.nro release/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libparam_hook.nro
mv libnro_hook.nro release/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libnro_hook.nro
mv libnn_hid_hook.nro release/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libnn_hid_hook.nro
cp target/aarch64-skyline-switch/release/libtraining_modpack.nro release/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libtraining_modpack.nro
ls -1 src/templates | xargs -n 1 basename | xargs -L1 -I{} cp src/templates/{} release/atmosphere/contents/01006A800016E000/manual_html/html-document/contents.htdocs/{}
mv colors.json release/colors.json