# -*- mode: dockerfile -*-

# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust or OpenSSL.
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
ARG ANGULAR_IMAGE=node:10.16.0-alpine

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder
# Add our source code.
ADD . ./
# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust/src/foos
RUN sudo chown -R rust:rust /home/rust/src/foos_web
RUN sudo chown -R rust:rust /home/rust/src/target
RUN sudo chown -R rust:rust /home/rust/src/Cargo.lock

# Build our application.
RUN cargo build --release --bin foos_web 

# Start the angular build environment
FROM ${ANGULAR_IMAGE} AS node
WORKDIR /usr/src/app
# Get package json
COPY foosClient/package*.json ./
# install npm on package.json
RUN npm install
# this comes after so as long as our dependencies didn't change we wouldn't have to wait for the npm install
COPY foosClient .
# build the angular app
RUN npm run build

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/foos_web /usr/local/bin/
COPY --from=node /usr/src/app/dist/ /foosClient/dist/
ENV PORT 12346
EXPOSE 12346
CMD /usr/local/bin/foos_web