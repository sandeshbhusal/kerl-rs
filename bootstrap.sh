#!/usr/bin/env bash
set -euo pipefail

PAC_DIR="rp2040_pac"
SVD_FILE="RP2040.svd"

rm -rf ${PAC_DIR}
cargo new --lib ${PAC_DIR}

cd ${PAC_DIR}
svd2rust -i ../${SVD_FILE} --target none

form -i lib.rs -o src/
rm lib.rs

cat <<EOF > Cargo.toml
[package]
name = "${PAC_DIR}"
version = "0.1.0"
edition = "2021"

[dependencies]
vcell = "0.1.3"
EOF

cargo fmt
cd ..
