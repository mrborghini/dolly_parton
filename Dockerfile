ARG APP_NAME=dolly_parton

FROM rust:alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git pkgconf libressl-dev

COPY . .

RUN cargo build --release && mv ./target/release/$APP_NAME /bin/server

FROM alpine:latest AS final

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# What the container should run when it is started.
CMD ["/bin/server"]
