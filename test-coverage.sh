#!/usr/bin/env bash

set -xe

OUTPUT_DIR=/tmp/serde_var_export
FIND_PREFIX=serde_var_export

CARGO_INCREMENTAL=0 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads" cargo +nightly test
mkdir -p ${OUTPUT_DIR}
zip -0 ${OUTPUT_DIR}/ccov.zip `find . \( -name "$FIND_PREFIX*.gc*" \) -print`
grcov ${OUTPUT_DIR}/ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" -o ${OUTPUT_DIR}/lcov.info
genhtml -o ${OUTPUT_DIR}/report/ --show-details --highlight --ignore-errors source --legend ${OUTPUT_DIR}/lcov.info
xdg-open ${OUTPUT_DIR}/report/index.html
