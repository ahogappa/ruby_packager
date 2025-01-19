FROM mcr.microsoft.com/devcontainers/base:ubuntu
# Install the xz-utils package
RUN apt-get update && apt-get install -y clang

WORKDIR /workspace

ENTRYPOINT [ "bash" ]
