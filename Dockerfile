FROM mcr.microsoft.com/devcontainers/base:ubuntu
# Install the xz-utils package
RUN apt-get update && apt-get install -y clang

WORKDIR /home

COPY a.out /home/a.out
RUN chmod +x /home/a.out
COPY main.rb /home/main.rb
