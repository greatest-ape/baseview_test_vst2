# baseview_test_vst2

Barebones [baseview](https://github.com/RustAudio/baseview)
[vst2](https://github.com/RustAudio/vst-rs) plugin that logs events to
`~/tmp/BaseviewTest.log`.

## Usage: macOS

- Run `scripts/macos-build-and-install.sh`
- Start your DAW, test the plugin

## Usage: Windows

- Run `cargo build`
- Copy `target/debug/libbaseview_test_vst2.dll` to your VST plugin folder
- Start your DAW, test the plugin

## Usage: Linux

- Run `cargo build`
- Copy `target/debug/libbaseview_test_vst2.so` to your VST plugin folder
- Start your DAW, test the plugin
