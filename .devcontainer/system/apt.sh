#!/usr/bin/env bash
# Installs apt-managed packages needed to build and work on whattf.how.
set -euo pipefail

apt-get update -y

# General purpose packages.
DEBIAN_FRONTEND=noninteractive apt-get install -y \
	ca-certificates \
	locales \
	pkg-config \
	tzdata

# Developer tools and build dependencies.
#   - build-essential: native build deps (a C toolchain) for crates whose build
#     scripts compile a little C.
#   - git / ssh: source control, including pushing to the Azure and GitHub remotes.
#   - xz-utils: extracting the Node.js .tar.xz tarball.
DEBIAN_FRONTEND=noninteractive apt-get install -y \
	build-essential \
	curl \
	git \
	jq \
	ripgrep \
	shellcheck \
	ssh \
	sudo \
	tar \
	tmux \
	tree \
	unzip \
	vim \
	wget \
	xz-utils \
	zip
