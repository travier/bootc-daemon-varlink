# bootc update daemon - Varlink version

**Notice: This is completely experimental and incomplete.**

## Goals

- Rust Varlink daemon/client with polkit access control
- Unprivileged operations for any active and interactive user:
    - Status
    - Check for update
- Unprivileged operations for any local, active and interactive user:
    - Update
- Privileged operations
    - None planed for now, use bootc directly instead

See:
- https://github.com/containers/bootc/issues/2
- https://github.com/containers/bootc/issues/4
- https://github.com/containers/bootc/issues/522

## How to

```
$ cargo build --bin bootc-daemon
$ sudo ./target/debug/bootc-daemon
...
```

```
$ cargo build --bin bootc-client
$ sudo chmod 777 /run/org.bootc.varlink
$ cargo run --bin bootc-client
...
```

# License

[MIT](LICENSES/MIT).
