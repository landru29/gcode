FROM rust:1.95.0-trixie

RUN apt update && apt install -y \
    build-essential \
    ca-certificates \
    dh-cargo \
    debhelper \
    devscripts \
    cmake \
    curl \
    git \
    pkg-config \
    unzip \
    vim \
    wget \
    zsh \
    sudo


RUN groupadd -g 1000 vscode && \
    useradd -rm -d /home/vscode -s /bin/zsh -g root -G sudo -u 1000 vscode && \
    echo "vscode:insecure" | chpasswd


WORKDIR /workspace

USER vscode

#!Install OhMyZsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended && \
    echo "source /usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh" >> ~/.zshrc && \
    echo "source /usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh" >> ~/.zshrc

RUN rustup component add rust-src clippy rustfmt
