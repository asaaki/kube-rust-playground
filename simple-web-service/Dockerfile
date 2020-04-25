# syntax=docker/dockerfile:experimental

# FUTURE: with buildkit support (currently experimental in Docker since 18.06) and
# https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/experimental.md
# we can potentially clean up a lot of cruft and utilize more caching.

## global args
ARG APP_NAME=simple-web-service
ARG BUILD_MODE=release
ARG STRIP=1
ARG COMPRESS=1

#########################
##### builder layer #####
#########################

# use specific hash to force correct platform (otherwise docker seems to choose randomly?);
# nope, the flag --platform=linux/amd64 was not helping!
FROM debian:buster-slim@sha256:1ceec96ca567c40500a2745728f7c19c0801785c8b10187b1d66bcd538694fc2 as builder

ENV DEBIAN_FRONTEND=noninteractive

ENV BUILD_CACHE_BUSTER="2020-04-25T00:00:00"

ENV RUSTUP_HOME=/usr/local/rustup \
  CARGO_HOME=/usr/local/cargo \
  PATH=/usr/local/cargo/bin:$PATH

ENV RUSTUP_INIT_URL="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
  RUST_TOOLCHAIN="stable" \
  DEB_PACKAGES="ca-certificates curl wget file patch make git xz-utils gcc-multilib musl-tools"

# @see https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/experimental.md#example-cache-apt-packages
RUN \
  --mount=type=cache,target=/var/cache/apt \
  --mount=type=cache,target=/var/lib/apt \
    set -eux; \
    uname -a; \
    rm -f /etc/apt/apt.conf.d/docker-clean; \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache; \
    \
    echo "Builder starts ..." \
 && apt-get update \
 && apt-get install -y --no-install-recommends $DEB_PACKAGES \
  \
 && wget $RUSTUP_INIT_URL \
 && chmod +x rustup-init \
 && ./rustup-init -y --no-modify-path --default-toolchain $RUST_TOOLCHAIN \
 && rm -f ./rustup-init \
 && chmod -R a+w $RUSTUP_HOME $CARGO_HOME \
 && rustup target add x86_64-unknown-linux-musl \
  \
 && wget -O upx.tar.xz https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz \
 && tar -xf upx.tar.xz --directory /bin --strip-components=1 $(tar -tf upx.tar.xz | grep -E 'upx$') \
 && rm -f upx.tar.xz \
  \
 && rustup --version \
 && cargo --version \
 && rustc --version \
 && echo Rust builder image done.

#######################
##### build layer #####
#######################

FROM builder as build

# @see https://docs.docker.com/engine/reference/builder/#understand-how-arg-and-from-interact
ARG APP_NAME
ARG BUILD_MODE
ARG STRIP
ARG COMPRESS

# create stub app for better build caching
RUN USER=root cargo new --bin /app

WORKDIR /app

COPY .cargo /app/.cargo
COPY Cargo.* /app/

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo fetch \
  && cargo build --${BUILD_MODE} --target=x86_64-unknown-linux-musl \
  && rm -rf /app/src

COPY build.rs /app/
COPY src /app/src

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  find src -exec touch {} + \
  && cargo build --${BUILD_MODE} --target=x86_64-unknown-linux-musl \
  && mkdir -p /app/build \
  && mv /app/target/x86_64-unknown-linux-musl/${BUILD_MODE}/${APP_NAME} .
# remove debug symbols
RUN [ "${STRIP}" = "1" ] && (echo "Stripping debug symbols ..."; strip ${APP_NAME}) || echo "No stripping enabled"
# compress binary; upx docs: https://github.com/upx/upx/blob/master/doc/upx.pod
RUN [ "${COMPRESS}" = "1" ] && (echo "Compressing binary ..."; upx --best ${APP_NAME}) || echo "No compression enabled"
RUN du -h ${APP_NAME}

RUN echo '#!/bin/sh' > /run-app \
 && echo "${APP_NAME} $@" >> /run-app \
 && chmod +x /run-app

######################
##### base layer #####
######################

FROM alpine:3.11 as base
RUN apk update --no-cache \
 && apk upgrade --no-cache \
 && apk add --no-cache tini
WORKDIR /app
ENTRYPOINT ["/sbin/tini", "--"]
CMD ["/bin/sh"]

####################
##### run layer ####
####################

FROM base as run
RUN addgroup -g 1001 appuser \
 && adduser  -u 1001 -G appuser -H -D appuser
USER 1001
ENV PATH "/app:${PATH}"

#######################
##### final image #####
#######################

FROM run as production

ARG APP_NAME
ARG DEFAULT_PORT=8080

LABEL tech.markentier.service.name="${APP_NAME}" \
      tech.markentier.service.repository="https://github.com/asaaki/kube-rust-playground"

ENV HOST=0.0.0.0 PORT=${DEFAULT_PORT}

COPY --from=build --chown=appuser:appuser /app/${APP_NAME} /app
COPY --from=build --chown=appuser:appuser /run-app /

EXPOSE ${DEFAULT_PORT}

CMD ["/run-app"]
