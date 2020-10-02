FROM clux/muslrust:stable

RUN cargo install --git https://github.com/jam1garner/cargo-skyline

RUN git clone https://github.com/jam1garner/rust-std-skyline-squashed

RUN cargo install xargo

RUN cd /volume/rust-std-skyline-squashed && cargo update

ENV XARGO_RUST_SRC /volume/rust-std-skyline-squashed/src

ENV PATH="/usr/share/rust/.rustup/toolchains/nightly-2020-04-10-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:${PATH}"

RUN mkdir /workspaces/
RUN ln -s /volume/rust-std-skyline-squashed /workspaces/rust-std-skyline-squashed