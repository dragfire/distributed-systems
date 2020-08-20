#! bash
rm -rf engine_*
cargo run --bin yakv-client -- set key1 value1 --addr 127.0.0.1:5000
cargo run --bin yakv-client -- get key1 --addr 127.0.0.1:5000 
cargo run --bin yakv-client -- set key1 value2 --addr 127.0.0.1:5000
cargo run --bin yakv-client -- get key1 --addr 127.0.0.1:5000
cargo run --bin yakv-client -- get key2 --addr 127.0.0.1:5000
cargo run --bin yakv-client -- rm key2 --addr 127.0.0.1:5000
cargo run --bin yakv-client -- set key2 value3 --addr 127.0.0.1:5000
cargo run --bin yakv-client -- rm key1 --addr 127.0.0.1:5000

sudo echo nameserver 9.9.9.11 >> /etc/resolvconf/resolv.conf.d/head