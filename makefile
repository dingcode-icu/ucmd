CARGO_BIN = cargo

BUILD_DIR  = "./build"

# ===make===

${CARGO_BIN} build  --rleease

# ===clean===
clean:
	rm -rf ${BUILD_DIR}
