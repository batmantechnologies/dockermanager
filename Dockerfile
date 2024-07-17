FROM registry.suse.com/bci/bci-base:latest
LABEL maintainer="Hayath"
LABEL version="1.0"
LABEL description="The Rust In Its Full Glory"

ARG USERNAME
ARG UID
ARG PROJECT_PWD

RUN zypper update -y && zypper install -y openssl-devel postgresql-devel
RUN zypper install -y --type pattern devel_basis

RUN useradd -ms /bin/bash --uid $UID $USERNAME; exit 0
RUN usermod -a -G sudo $USERNAME; exit 0
RUN usermod -a -G users $USERNAME; exit 0
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER $USERNAME
ENV USER=$USERNAME

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=$PATH:/home/$USERNAME/.local/bin/:/home/$USERNAME/.cargo/bin/

WORKDIR "$PROJECT_PWD"

# Below is optional
#########################################
# for diesel_cli
# RUN cargo install diesel_cli
# RUN cargo install diesel_cli --no-default-features --features "postgres"
#########################################
