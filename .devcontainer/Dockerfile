# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.195.0/containers/rust/.devcontainer/base.Dockerfile
# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

# RUN cargo install

RUN cargo install mdbook
RUN cargo install mdbook-svgbob
RUN cargo install mdbook-i18n-helpers
RUN git clone https://github.com/accelerant-dev/comprehensive-rust/ &&\
    cd comprehensive-rust &&\
    git checkout 2023-10 &&\
    cargo install --path mdbook-exerciser &&\
    cargo install --path mdbook-course
