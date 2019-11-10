FROM rustlang/rust:nightly
WORKDIR /app

RUN apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv 7F0CEB10 \
&& echo "deb http://repo.mongodb.org/apt/ubuntu xenial/mongodb-org/3.4 multiverse" | tee /etc/apt/sources.list.d/mongodb-org-3.4.list \
&& apt update -y && apt install mongodb

COPY *.toml .
COPY Cargo.lock .
RUN cargo build --release

COPY src src
RUN cargo build --release

EXPOSE 8080
CMD cargo run --release