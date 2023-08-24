# Change these to match your local
# The first time you run this, in order to set up the symlinks, you may have to be an administrator
# to write the files. Powershell is dumb.
$RYUJINX_LAYOUT_ARC_PATH="C:\Users\Josh\AppData\Roaming\Ryujinx\sdcard\ultimate\TrainingModpack\layout.arc"
$LOCAL_LAYOUT_ARC_PATH="C:\Users\Josh\Documents\Games\UltimateTrainingModpack\src\static\layout.arc"

$RYUJINX_PLUGIN_PATH="C:\Users\Josh\AppData\Roaming\Ryujinx\mods\contents\01006a800016e000\romfs\skyline\plugins\libtraining_modpack.nro"
$LOCAL_PLUGIN_PATH="C:\Users\Josh\Documents\Games\UltimateTrainingModpack\target\aarch64-skyline-switch\release\libtraining_modpack.nro"

$RYUJINX_EXE_PATH="C:\Users\Josh\Documents\Games\Ryujinx\ryujinx-1.1.999-win_x64\publish\Ryujinx.exe"
$SMASH_NSP_PATH='C:\Users\Josh\Documents\Games\ROMs\Super Smash Bros Ultimate [Base Game]\Super Smash Bros Ultimate[01006A800016E000][US][v0].nsp'


$IP=(Test-Connection -ComputerName (hostname) -Count 1  | Select -ExpandProperty IPV4Address).IPAddressToString

# Set symbols flag
$env:RUSTFLAGS="-g"
cargo skyline build --release --features layout_arc_from_file
if (($lastexitcode -ne 0)) {
    exit $lastexitcode
}

# Set up symlinks
if(-not(Test-path $RYUJINX_LAYOUT_ARC_PATH -PathType leaf))
{
    New-Item -ItemType SymbolicLink -Path $RYUJINX_LAYOUT_ARC_PATH -Target $LOCAL_LAYOUT_ARC_PATH
    if (($lastexitcode -ne 0)) {
        exit $lastexitcode
    }
}

if(-not(Test-path $RYUJINX_PLUGIN_PATH -PathType leaf))
{
    New-Item -ItemType SymbolicLink -Path $RYUJINX_PLUGIN_PATH -Target $LOCAL_PLUGIN_PATH
    if (($lastexitcode -ne 0)) {
       exit $lastexitcode
    }
}

try {
    # Start the process asynchronously
    $process = Start-Process -FilePath $RYUJINX_EXE_PATH -ArgumentList `"$SMASH_NSP_PATH`" -PassThru

    # Store the process ID
    $global:process = $process.Id

    echo "Starting cargo skyline listen..."
    cargo skyline listen --ip=$IP
    # Makes no sense, but we need this line for logs to show up. Lol
    echo "Finishing cargo skyline listen..."
}
finally {
    # Interrupts to the script should kill Ryujinx as well
    if ($global:process -ne $null) {
        Stop-Process -Id $global:process -Force
    }
}
