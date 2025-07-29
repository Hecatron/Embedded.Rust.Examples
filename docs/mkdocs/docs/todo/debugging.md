# Debugging

## Windows debugging

Windows has some pathing issues when debugging with probe-rs using 0.27
There has been a pull request with a fix
<https://github.com/probe-rs/probe-rs/pull/3137>
but a stable release hasn't been released yet that contains it
Currently using git commit cc51528

To switch from the stable to the latest git release for probe-rs-tools
```bash
cargo uninstall probe-rs-tools
cargo install --git https://github.com/probe-rs/probe-rs.git --branch master probe-rs-tools
```

## Breakpoints

There seems to be an issue with probe-rs and debugging

  * When debugging with the pico probe
    If Step into or Step Over is used, then breakpoints stop working?

  * Might be a probe-rs issue considering master is being used
  * Try Linux with 0.27
  * Try using the segger instead of picoprobe
