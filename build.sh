# build.sh
out_dir=target/release/data
if [[ -d "$out_dir" ]]; then
  rm -r "$out_dir"
fi
mkdir -p "$out_dir"
cargo build --release