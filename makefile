CARGO_BIN = cargo
BIN_OUTPUT = ucmd

PROJ_DIR = "./proj"
PROJ_BUILD_DIR  = "./proj/target/release/"${BIN_OUTPUT}
PORJ_WIN_DIR= "./proj/target/x86_64-pc-windows-gnu/release/${BIN_OUTPUT}.exe"

# ===make===
${BIN_OUTPUT}:
	cd ${PROJ_DIR} && ${CARGO_BIN} build  --release
	upx ${PROJ_BUILD_DIR}
# ===clean===
clean:

	rm -rf ${BUILD_DIR}

windows:
	cd ${PROJ_DIR} && ${CARGO_BIN} build  --release  --target x86_64-pc-windows-gnu
	upx ${PORJ_WIN_DIR}