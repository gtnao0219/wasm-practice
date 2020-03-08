FROM rust:1.41

RUN cargo install cargo-generate
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash - \
  && apt-get install -y nodejs \
  && apt-get clean