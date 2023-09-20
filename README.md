# spacex_log

One log formatter with env_logger.

## Example

Run as `cargo run --example simple`.

```rust
use log::{debug, error, info, warn};
use spacex_log::init_log;

fn main() {
    init_log("INFO");
    // init_log("info");

    debug!("debug info");

    info!("test info");

    warn!("warn info");

    error!("error");
}
```

Then logs:

```bash
2023-09-20 15:23:17 [INFO] [simple]: test info
2023-09-20 15:23:17 [WARN] [simple]: warn info
2023-09-20 15:23:17 [ERROR] [simple]: error
```

If you want to disable log colors, you can set the env `NO_COLOR` to `true`.

> PS: publish by `cargo publish --registry crates-io`.
