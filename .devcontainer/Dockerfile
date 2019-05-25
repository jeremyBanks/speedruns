FROM rust:1

# Copy endpoint specific user setting overrides into container
COPY settings.vscode.json /root/.vscode-remote/data/Machine/settings.json

# Update Rustup
RUN rustup update 2>&1

# Configure apt
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils 2>&1

# Install git, process tools, lsb-release (common in install instructions for CLIs)
RUN apt-get -y install git procps lsb-release

# Install other dependencies
RUN apt-get install -y lldb-3.9

# Clean up
RUN apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
ENV DEBIAN_FRONTEND=dialog

# Set the default shell to bash rather than sh
ENV SHELL /bin/bash

# Install the version of Rust specified in rust-toolchain, with required components
COPY rust-toolchain /rust-toolchain
RUN rustup toolchain install "$(cat "/rust-toolchain")"
RUN rustup component add --toolchain "$(cat "/rust-toolchain")" rls rust-analysis rust-src rustfmt clippy 2>&1

# Expose ports used by the application
EXPOSE 59330-59333
