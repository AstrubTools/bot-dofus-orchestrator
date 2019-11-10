FROM rustlang/rust:nightly
WORKDIR /app

RUN \
   apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv 7F0CEB10 && \
   echo 'deb http://downloads-distro.mongodb.org/repo/ubuntu-upstart dist 10gen' | sudo tee /etc/apt/sources.list.d/mongodb.list && \
   apt-get update && \
   apt-get install -y mongodb-org

COPY *.toml .
COPY Cargo.lock .
RUN cargo build --release

COPY src src
RUN cargo build --release

EXPOSE 8080
CMD cargo run --release