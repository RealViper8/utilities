build:
	cargo build
build-windows:
	@brew || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
	clear
	@cargo build --target x86_64-pc-wwindows-gnu && echo "\n\x1b[0;32mSuccesfully Compiled for Windows!\x1b[0m\n" || rustup target add x86_64-pc-windows-gnu && brew install mingw-w64
setup-rust:
	@cargo || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	clear
	@cargo r || echo failed
clean:
	@rm -r target || echo target not found