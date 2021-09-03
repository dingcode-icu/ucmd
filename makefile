CARGO_BIN = cargo
BIN_OUTPUT = ucmd

PROJ_DIR = "./proj"
PROJ_BUILD_DIR  = "./proj/target/release/"${BIN_OUTPUT}


# ===make===
${BIN_OUTPUT}:
	cd ${PROJ_DIR} && ${CARGO_BIN} build  --release
	upx ${PROJ_BUILD_DIR}
# ===clean===
clean:

	rm -rf ${BUILD_DIR}
