FROM fedora:latest

RUN dnf install -y clang llvm libbpf-devel
RUN dnf install -y iproute iputils procps-ng
RUN dnf install -y git make 
RUN dnf clean all

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Cargo bin to PATH for container execution
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup toolchain install nightly --component rust-src

RUN cargo install bpf-linker

## custom dx comfort, please feel free to comment out if you prefer plain ls
RUN dnf install -y eza vim neovim
RUN echo "alias ls='eza --icons'" >> /root/.bashrc && \
    echo "alias l='eza -l --icons --git'" >> /root/.bashrc && \
    echo "alias ll='eza -l --icons'" >> /root/.bashrc && \
    echo "alias la='eza -a --icons'" >> /root/.bashrc && \
    echo "alias lla='eza -la --icons'" >> /root/.bashrc

WORKDIR /workspace
