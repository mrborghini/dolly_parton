ARG APP_NAME=dolly_parton

FROM rust:alpine AS build
ARG APP_NAME
WORKDIR /app

COPY .env .
COPY system_message.txt .

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git pkgconf libressl-dev

COPY . .

RUN cargo build --release && mv ./target/release/$APP_NAME /usr/bin/server

# Add a new stage to handle the conditional copying
FROM alpine:latest AS final

# Copy the executable from the "build" stage.
COPY --from=build /usr/bin/server /usr/bin/

# Check if the system_message.txt exists and copy it if it does
COPY --from=build /app/system_message.txt /
COPY --from=build /app/.env /

# What the container should run when it is started.
CMD ["/usr/bin/server"]
