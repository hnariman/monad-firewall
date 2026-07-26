FROM fedora:latest

## dev
RUN dnf install -y clang llvm libbpf-devel
## net-dev
RUN dnf install -y iproute iputils procps-ng

## needed for vm env lima etc
RUN dnf install -y qemu-system-x86 qemu-img genisoimage openssh-clients tar jq

RUN dnf clean all

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Cargo bin to PATH for container execution
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup toolchain install nightly --component rust-src

RUN cargo install bpf-linker

# we have qcow2 once and cache in container layer
RUN mkdir -p /opt/lima-cache && \
    curl -L -o /opt/lima-cache/arch.qcow2 \
    "https://geo.mirror.pkgbuild.com/images/latest/Arch-Linux-x86_64-cloudimg.qcow2"

RUN LIMA_VERSION=$(curl -fsSL https://api.github.com/repos/lima-vm/lima/releases/latest | jq -r .tag_name) && \
    curl -fsSL "https://github.com/lima-vm/lima/releases/download/${LIMA_VERSION}/lima-${LIMA_VERSION:1}-Linux-x86_64.tar.gz" | tar Cxzvm /usr/local

COPY lima/*.qcow2 /opt/lima-cache/Arch-Linux-x86_64-cloudimg.qcow2
COPY lima/mini-vm.yml /etc/lima/mini-vm.yml

RUN dnf install -y eza vim neovim
RUN echo "alias ls='eza --icons'" >> /root/.bashrc && \
    echo "alias l='eza -l --icons --git'" >> /root/.bashrc && \
    echo "alias ll='eza -l --icons'" >> /root/.bashrc && \
    echo "alias la='eza -a --icons'" >> /root/.bashrc && \
    echo "alias lla='eza -la --icons'" >> /root/.bashrc

RUN useradd -m -s /bin/bash developer && \
    echo "developer ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER developer
WORKDIR /home/developer


WORKDIR /workspace
