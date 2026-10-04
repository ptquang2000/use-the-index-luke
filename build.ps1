New-Item -ErrorAction Ignore -ItemType Directory -Path $PSScriptRoot/build
Push-Location $PSScriptRoot/build

rustc --error-format=short -g -C opt-level=0 -A unused --out-dir . $PSScriptRoot/win32_util.rs --crate-type bin

Pop-Location
