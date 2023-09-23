FROM rust

LABEL maintainer="Hayath"
LABEL version="1.0"
LABEL description="The Rust In Its Full Glory"

ARG USERNAME
ARG UID
ARG PROJECT_PWD

RUN apt-get remove git -y && apt-get update -y
# Below is optional
#########################################
#RUN apt-get install -y tree
#RUN apt-get install -y postgresql-client-common postgresql-client
RUN rustup update && rustup install stable
#########################################

RUN useradd -ms /bin/bash $USERNAME -u $UID; exit 0
RUN usermod -a -G sudo $USERNAME; exit 0
USER $USERNAME
ENV USER=$USERNAME
ENV PATH=$PATH:/home/$USERNAME/.local/bin/

WORKDIR "$PROJECT_PWD"
# Below is optional
#########################################
# for diesel_cli
RUN cargo install diesel_cli
#########################################
