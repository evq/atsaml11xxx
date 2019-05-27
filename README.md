# atsaml11xxx

Peripheral access API for SAM L11 microcontrollers

(generated using svd2rust)

# WIP

This crate is a work in progress.

## Usage

### Building the blink example

You'll need a recent Rust `nightly` build. Add the `thumbv8m.base-none-eabi` target:

```
$ rustup target add thumbv8m.base-none-eabi
```

Then use cargo to build the example:

```
$ cargo build --release --example blink
```

### Programming blink example to FLASH on the SAM L11 XPLAINED dev board

First build and install [edbg](https://github.com/ataradov/edbg), SAM L11
support was recently added.

After building per the above section:

```
arm-none-eabi-objcopy target/thumbv8m.base-none-eabi/release/examples/blink target/thumbv8m.base-none-eabi/release/examples/blink.bin -O binary
edbg -t mchp_cm23 -e -p -f target/thumbv8m.base-none-eabi/release/examples/blink.bin
```

### Running blink example out of RAM on the SAM L11 XPLAINED dev board

This requires RAM to be marked as executable, for help doing this see:
https://github.com/dwelch67/atsaml11_samples/tree/master/ATSAML11

You will want to edit `.cargo/config`, change `rustflags` as follows, then rebuild.
```
rustflags = [
  "-C", "link-arg=-Tram.x",
]
```

This seems to only work with openocd 0.9, in one window run:
```
openocd -f saml11_xplained_openocd.cfg
```

In another:
```
arm-none-eabi-gdb target/thumbv8m.base-none-eabi/release/examples/blink
```
