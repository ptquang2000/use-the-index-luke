ROOT=$(pwd)

mkdir -p ./build
pushd build >/dev/null

rustc --error-format=short -g -C opt-level=0 -A unused -C link-args=-Wl,-rpath -C link-args=-Wl,-rpath,$ORIGIN --out-dir . $ROOT/linux_util.rs --crate-type bin

popd >/dev/null
