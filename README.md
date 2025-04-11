# Ntfy-send

Ntfy-send is a small (binary is 5.2 Mb) app written in Rust, that sends Ntfy push notification messages. It has implemented queue (if there is no internet connection, messages are saved to queue file and could be send to the server, when internet connection is established. It logs important events (mostly if message was sent successfully or not) and has implemented logrotation to save space.

ARM32 binary: [ntfy-send](ntfy-send).

## Project structure
```
ntfy-send/
├── Cargo.toml
├── .cargo/
│   └── config.toml
└── src/
    ├── main.rs
    ├── logging.rs
    └── queue.rs
```

## Install cross-compilation tools:
```
cd rayhunter
wget https://musl.cc/armv7l-linux-musleabihf-cross.tgz
sudo tar -xzf armv7l-linux-musleabihf-cross.tgz -C /opt
export PATH=/opt/armv7l-linux-musleabihf-cross/bin:$PATH
```

### Verify installation:
```
armv7l-linux-musleabihf-gcc --version
```

## Compiling
```
cd ntfy-send
```

### Add the ARM target
```
rustup target add armv7-unknown-linux-musleabihf
```

`nano .cargo/config`:
```
[target.armv7-unknown-linux-musleabihf]
linker = "armv7l-linux-musleabihf-gcc"
```

### Build the application
```
PATH="/opt/armv7l-linux-musleabihf-cross/bin:$PATH" \
CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=armv7l-linux-musleabihf-gcc \
cargo build --release --target=armv7-unknown-linux-musleabihf
```

### Check the file
```
file target/armv7-unknown-linux-musleabihf/release/ntfy-queue
```
Should be ELF 32-bit LSB executable and statically linked:
```
target/armv7-unknown-linux-musleabihf/release/ntfy-send: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped
```

## Install `ntfy-send` on the device

### Copy the binary file to the device 
```
adb push target/armv7-unknown-linux-musleabihf/release/ntfy-queue /media/card/ntfy-queue
```

### Create log folder
```
adb shell
```

```
mkdir -p /var/lib/ntfy-send/
```

If you want to change logging directory, you must change file `logging.rs`:
```
const LOG_FILE: &str = "/var/lib/ntfy-send/ntfy-send.log";
```
and in file `queue.rs`:
```
const QUEUE_DIR: &str = "/var/lib/ntfy-send";
const QUEUE_FILE: &str = "unsent-messages-queue.json";
```

## Running
```
cd /media/card
ntfy-send v1.0
```
```
Usage:
  Send message: ntfy-send -s SERVER -t TOPIC -m MESSAGE [-u USERNAME -p PASSWORD]
  Clear queue: ntfy-send clear-queue
```

### Sending messages
```
./media/card/ntfy-send -s https://ntfy.envs.net -t YourTopic -m "Hello from TP_Link M7350."
```

### Clearing queue

Enable cron:
`vi /etc/init.d/rc`, at the end add:
```
crond
```
Run `crontab -e` and add:
```
*/5 * * * * /media/card/ntfy-send clear-queue
```
This will try to clear queue at 5 minute intervals.

### Example: notify when Rayhunter starts

```
#!/bin/sh
if [ -f "/tmp/rayhunter.pid" ]; then
    /media/card/ntfy-send -s https://ntfy.envs.net -t YourTopic -m "Rayhunter is running."
else
    /media/card/ntfy-send -s https://ntfy.envs.net -t YourTopic -m "ERROR: Rayhunter is not running!"
fi
```

### Logs

Ntfy-send log file:
```
cat /var/lib/ntfy-send/ntfy-send.log
```

Logrotate is currently set to 500 entries, you can change this in file `logging.rs`:

```
const MAX_LOG_ENTRIES: usize = 500; // Set to 5 for only last 5 entries
```

Messages in queue that were not sent yet:
```
cat /var/lib/ntfy-send/unsent-messages-queue.json
```
