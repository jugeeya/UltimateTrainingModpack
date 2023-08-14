$IP=(Test-Connection -ComputerName (hostname) -Count 1  | Select -ExpandProperty IPV4Address).IPAddressToString
cargo skyline build --release --features layout_arc_from_file
if (($lastexitcode -ne 0)) {
    exit $lastexitcode
}

# Set up symlinks
$RYUJINX_LAYOUT_ARC_PATH="C:\Users\Josh\AppData\Roaming\Ryujinx\sdcard\ultimate\TrainingModpack\layout.arc"
$LOCAL_LAYOUT_ARC_PATH="C:\Users\Josh\Documents\Games\UltimateTrainingModpack\src\static\layout.arc"
if(-not(Test-path $RYUJINX_LAYOUT_ARC_PATH -PathType leaf))
{
    New-Item -ItemType SymbolicLink -Path $RYUJINX_LAYOUT_ARC_PATH -Target $LOCAL_LAYOUT_ARC_PATH
}

$RYUJINX_PLUGIN_PATH="C:\Users\Josh\AppData\Roaming\Ryujinx\mods\contents\01006a800016e000\romfs\skyline\plugins\libtraining_modpack.nro"
$LOCAL_PLUGIN_PATH="C:\Users\Josh\Documents\Games\UltimateTrainingModpack\target\aarch64-skyline-switch\release\libtraining_modpack.nro"
if(-not(Test-path $RYUJINX_PLUGIN_PATH -PathType leaf))
{
    New-Item -ItemType SymbolicLink -Path $RYUJINX_PLUGIN_PATH -Target $LOCAL_PLUGIN_PATH
}

C:\Users\Josh\Documents\Games\Ryujinx\publish\Ryujinx.exe "C:\Users\Josh\Documents\Games\ROMs\Super Smash Bros Ultimate [Base Game]\Super Smash Bros Ultimate[01006A800016E000][US][v0].nsp"
cargo skyline listen --ip=$IP
# C:\Users\Jdsam\Documents\Games\Emulators\ryujinx-1.1.299-win_x64\publish\BLAH.exe C:\Users\Jdsam\Documents\Games\Emulators\ryujinx-1.1.299-win_x64\publish\Ryujinx.exe "C:\Users\Jdsam\Documents\Games\SmashRoms\UltimateXCI\ultimate.xci"