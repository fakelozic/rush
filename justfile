# https://just.systems

default:
    echo 'Hello, world!'
clippy-r:
    cargo clippy --release
main-r:
    cargo run --bin rush --release

learn-r:
    cargo run --bin learn --release