FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-22.04

ARG USERNAME=vscode
ARG NOVNC_VERSION=1.2.0
ARG WEBSOCKIFY_VERSION=0.9.0
ENV DEBIAN_FRONTEND noninteractive

ARG WIDHT=1024
ARG HEIGHT=768

ENV DISPLAY :0
ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US.UTF-8 \
    LC_ALL=C.UTF-8 \
    DISPLAY_WIDTH=${WIDHT} \
    DISPLAY_HEIGHT=${HEIGHT}

# C++
# python
# node
# SDL2
# OpenGL
# Qt5
# GTK4
RUN apt-get update && apt-get install -y --no-install-recommends build-essential manpages-dev \
    git unzip dosfstools curl wget vim sudo ssh tzdata ca-certificates \
    xauth uuid-dev \
    dbus dbus-x11 alsa-utils pulseaudio pulseaudio-utils mesa-utils x11-apps \
    cmake clang clang-format clangd gdb nasm lldb\
    python3 python-is-python3 python3-distutils python3-numpy python3-pip python3-tk \
    # nodejs \
    # libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-net-dev libsdl2-ttf-dev \
    # libglu1-mesa-dev freeglut3-dev mesa-common-dev libglew-dev libglfw3-dev \
    # qtcreator qtbase5-dev qttools5-dev-tools qtbase5-examples qt5-qmake \
    # libgtk-4-bin libgtk-4-common libgtk-4-dev \
    xvfb x11vnc xdotool wget tar supervisor net-tools lxde xterm \
    && apt-get clean -y && rm -rf /var/lib/apt/lists


# Install noVNC
RUN mkdir -p /usr/local/novnc \
 && curl -k -L -o /tmp/novnc.zip https://github.com/novnc/noVNC/archive/v${NOVNC_VERSION}.zip \
 && unzip /tmp/novnc.zip -d /usr/local/novnc/ \
 && cp /usr/local/novnc/noVNC-${NOVNC_VERSION}/vnc.html /usr/local/novnc/noVNC-${NOVNC_VERSION}/index.html \
 && curl -k -L -o /tmp/websockify.zip https://github.com/novnc/websockify/archive/v${WEBSOCKIFY_VERSION}.zip \
 && unzip /tmp/websockify.zip -d /usr/local/novnc/ \
 && ln -sf /usr/local/novnc/websockify-${WEBSOCKIFY_VERSION} /usr/local/novnc/noVNC-${NOVNC_VERSION}/utils/websockify \
 && rm -rf /tmp/novnc.zip /tmp/websockify.zip

RUN cd /usr/bin && sudo mv lxpolkit lxpolkit.bak

ADD supervisord.conf /etc/supervisor/conf.d/supervisord.conf

USER ${USERNAME}

# Install Rust
ENV RUST_VERSION stable
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH $PATH:/home/${USERNAME}/.cargo/bin
RUN rustup install stable
RUN rustup install nightly
RUN rustup component add rust-analysis rust-src rustfmt clippy
RUN cargo install cargo-watch
RUN cargo install cargo-check
# Install FireDBG
# RUN curl https://raw.githubusercontent.com/SeaQL/FireDBG.for.Rust/main/install.sh -sSf | sh

RUN mkdir -p /home/${USERNAME}/workspace
WORKDIR /home/${USERNAME}/workspace

USER root
COPY ./.inject_bashrc /root
RUN cat /root/.inject_bashrc >> /home/${USERNAME}/.bashrc

EXPOSE 8085
CMD ["/usr/bin/supervisord"]


