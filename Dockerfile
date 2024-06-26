# This is an example build stage for the node template. Here we create the binary in a temporary image.

# This is a base image to build substrate nodes
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /template-node
COPY . .
RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the binary."
FROM docker.io/library/ubuntu:24.04

# Copy the node binary.
COPY --from=builder /template-node/target/release/template-node /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /node-dev node-dev && \
  mkdir -p /chain-data /node-dev/.local/share && \
  chown -R node-dev:node-dev /chain-data && \
  ln -s /chain-data /node-dev/.local/share/template-node && \
  # unclutter and minimize the attack surface
  rm -rf /usr/bin /usr/sbin && \
  # check if executable works in this container
  /usr/local/bin/template-node --version

USER node-dev

EXPOSE 30333 9933 9944 9615
VOLUME ["/chain-data"]

ENTRYPOINT ["/usr/local/bin/template-node"]
