FROM rust

LABEL maintainer="Hayath"
LABEL version="1.0"
LABEL description="The Rust In Its Full Glory"

ARG USERNAME
ARG UID
ARG PWD

RUN apt-get update -y

# Below is optional
#########################################
#RUN apt-get install -y tree
#RUN apt-get install -y postgresql-client-common postgresql-client
RUN rustup update && rustup install stable
#########################################

RUN apt-get remove git -y
RUN apt-get update -y

RUN useradd -ms /bin/bash $USERNAME -u $UID; exit 0
RUN usermod -a -G sudo $USERNAME; exit 0
WORKDIR "$PWD"
EXPOSE 8000
USER $USERNAME
ENV USER=$USERNAME
ENV PATH=$PATH:/home/$USERNAME/.local/bin/

# Below is optional
#########################################
# for diesel_cli
RUN cargo install diesel_cli
#########################################
