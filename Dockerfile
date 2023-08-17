FROM docker.io/rust:slim-bookworm

RUN apt-get update --fix-missing \
    && apt-get -y install --no-install-recommends \
        bash \
        jq \
        curl \
        build-essential \
        gosu \
        git \
        pkg-config \
    && apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

ARG HOST_USER
ARG HOST_UID
ARG HOST_GID

RUN (grep ${HOST_GID} /etc/group || groupadd -g ${HOST_GID} ${HOST_USER}) \
    && useradd -u ${HOST_UID} -g ${HOST_GID} -m ${HOST_USER} \
    && mkdir -p /html \
    && chown -R ${HOST_USER}:${HOST_GID} /html

USER ${HOST_USER}

RUN cargo install \
    basic-http-server \
    cargo-watch

RUN rustup target add wasm32-unknown-unknown \
    && rustup component add rustfmt \
    && rustup component add clippy

COPY files/watchme.sh .
COPY files/bash_prompt.sh /etc/profile.d/
COPY files/profile /etc/profile

WORKDIR /app

CMD ["/bin/sh", "/watchme.sh" ]
