make:
	bash -c '\
		. ~/.nvm/nvm.sh; \
		set -veuxo pipefail; \
		type nvm || (echo "ERROR: please install nvm from https://github.com/nvm-sh/nvm"; exit 1); \
		type rustup || (echo "ERROR: please install rustup https://rustup.rs/"; exit 1); \
		nvm install; \
		nvm use; \
		rustup install "$$(cat rust-toolchain)"; \
		yarn install; \
		cargo build --workspace; \
		yarn start; \
	'