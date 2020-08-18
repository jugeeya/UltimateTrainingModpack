# assumes you've set ip with `cargo skyline set-ip [x.x.x.x]`

cargo skyline build --release && cargo skyline install
cd TrainingModpackOverlay && make && cd - && cargo skyline cp TrainingModpackOverlay/ovlTrainingModpack.ovl sd:/switch/.overlays