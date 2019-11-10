
# bot-dofus-orchestrator

[![Try it on gitpod](https://img.shields.io/badge/try-on%20gitpod-brightgreen.svg)](https://gitpod.io/#https://github.com/The-Tensox/bot-dofus-orchestrator)
[![Build Status](https://img.shields.io/circleci/project/The-Tensox/bot-dofus-orchestrator/master.svg)](https://circleci.com/gh/The-Tensox/bot-dofus-orchestrator)

- REST API to access bot-dofus-orchestrator data

## Installation

```bash
sudo apt update
sudo apt install mongodb
```

Check mongodb status

```bash
service mongodb status
```

```bash
echo -e "MONGO_ADDR=localhost
DB_NAME=bot-dofus-orchestrator
MONGO_PORT=27017" > .env
```

## Deployment on Google Cloud

```bash
gcloud app deploy
```

## Usage

```bash
cargo run &

# If API key required add --header "x-api-key: valid_api_key"

# POST
curl -d '{"task": "LoginToAccount"}' -H "Content-Type: application/json" -X POST http://localhost:8001/tasks
# PUT
curl -d '{"$oid": "5db15a686539303d5708901f", "task": "LoginToCharacter"}' -H "Content-Type: application/json" \
-X PUT http://localhost:8001/tasks/5db15a686539303d5708901f

# GET
curl http://localhost:8001/tasks
# Find by id
curl http://localhost:8001/tasks/5db15a1f6539303d5708901e

# DELETE
curl -H "Content-Type: application/json" -X DELETE http://localhost:8001/tasks/5db15a1f6539303d5708901e

# DELETE all
curl -X DELETE localhost:8001/tasks
```

## Tests

To avoid running parallel tests we use --test-threads=1 because we modify database, otherwise tests would fail.

```rust
cargo test -- --test-threads=1
```

## Contribute

- Use cargo fmt

## TODO

 - [x] When a POST / PUT is done notify the (created / updated) value to the connected clients via websocket
 - [ ] Benches [see example](https://bheisler.github.io/criterion.rs/book/getting_started.html)
 - [x] Tests
 - [ ] Documentation [see](https://doc.rust-lang.org/rust-by-example/meta/doc.html)