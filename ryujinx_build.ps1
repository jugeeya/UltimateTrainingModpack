$IP=(Test-Connection -ComputerName (hostname) -Count 1  | Select -ExpandProperty IPV4Address).IPAddressToString
cargo skyline build --release
Copy-Item target/aarch64-skyline-switch/release/libtraining_modpack.nro 'C:\Users\Jdsam\AppData\Roaming\Ryujinx\mods\contents\01006A800016E000\romfs\skyline\plugins\'
cargo skyline listen --ip=$IP