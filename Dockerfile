ARG APP_NAME=dolly_parton

FROM rust:alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git pkgconf libressl-dev

COPY . .

# Check if the system_message.txt exists. If it doesn't take the example file
RUN if ! test -f ./system_message.txt; then cp ./system_message_example.txt ./system_message.txt; fi

RUN cargo build --release && mv ./target/release/$APP_NAME /usr/bin/server

# Add a new stage to handle the conditional copying
FROM alpine:latest AS final

# Copy the executable from the "build" stage.
COPY --from=build /usr/bin/server /usr/bin/

# Create out_dir
RUN mkdir out_dir

# Copy the example text to / as a backup
COPY --from=build /app/system_message_example.txt /

# Copy system_message.txt to /out_dir
COPY --from=build /app/system_message.txt /out_dir

# What the container should run when it is started.
CMD ["/usr/bin/server"]
