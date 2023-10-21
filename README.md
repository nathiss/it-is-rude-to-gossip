# It's rude to gossip

This repository contains solution for _some_ of the challenges from [Gossip Glomers](https://fly.io/dist-sys/).

- [x] [Echo](https://fly.io/dist-sys/1/)
- [x] [Unique ID Generation](https://fly.io/dist-sys/2/)
- [ ] [Broadcast](https://fly.io/dist-sys/3a/)
- [ ] [Grow-Only Counter](https://fly.io/dist-sys/4/)
- [ ] [Kafka-Style Log](https://fly.io/dist-sys/5a/)
- [ ] [Totally-Available Transactions](https://fly.io/dist-sys/6a/)

## Usage

Run the program with

```sh
cargo r
```

It listens for [maelstrom](https://github.com/jepsen-io/maelstrom) messages serialized to
[JSONL](https://jsonlines.org/) on stdin. Full protocol specification can be found here:
[maelstrom/doc/protocol.md](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md).

## License

MIT
