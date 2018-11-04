# atsaml11xxx

Peripheral access API for SAM L11 microcontrollers

(generated using svd2rust)

# WIP

This crate is a work in progress. Building it either requires
a xargo change or renaming the target spec due to the target
naming convention used.

https://github.com/japaric/xargo/pull/226

It makes use of changes to `cortex-m-rt` and `cortex-m-semihosting`
which have not yet been merged upstream.

By default the linker is configured to run everything out of RAM,
I have not yet managed to program the flash on this chip.

For help enabling executable RAM or more hints at how to program the flash see:

https://github.com/dwelch67/atsaml11_samples/tree/master/ATSAML11

## Usage

### Building the blink example

```
RUST_TARGET_PATH=`pwd` xargo build --release --target thumbv8m.base-none-eabi --example blink
```

### Running blink example on the SAM L11 XPLAINED dev board

This seems to only work with openocd 0.9, in one window run:
```
openocd -f saml11_xplained_openocd.cfg
```

In another:
```
arm-none-eabi-gdb target/thumbv8m.base-none-eabi/release/examples/blink
```
