if [ "$TRAVIS_RUST_VERSION" = "stable" ];
then
    rustup target add x86_64-unknown-linux-musl
    cargo build --bin qdht --release --target x86_64-unknown-linux-musl
    cp target/x86_64-unknown-linux-musl/release/qdht qdht-$TRAVIS_TAG-x86_64-linux
fi