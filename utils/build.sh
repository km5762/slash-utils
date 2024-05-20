crates=$(find . -type d -exec test -e "{}/Cargo.toml" ';' -print)

for crate in $crates; do
    echo "Building crate: $crate"
    wasm-pack build $crate --target web
done
