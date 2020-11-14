# baseview_test_vst2

Barebones [baseview](https://github.com/RustAudio/baseview)
[vst2](https://github.com/RustAudio/vst-rs) plugin that logs events to
`~/tmp/BaseviewTest.log`.

## Usage: macOS

- Make sure baseview repo is in ../baseview
- Run `scripts/macos-build-and-install.sh`
- Start your DAW, test the plugin

## Usage: Windows

- Make sure baseview repo is in ../baseview
- Run `cargo build`
- Copy `target/debug/libbaseview_test_vst2.dll` to your VST plugin folder
- Start your DAW, test the plugin

## Usage: Linux

- Make sure baseview repo is in ../baseview
- Run `cargo build`
- Copy `target/debug/libbaseview_test_vst2.so` to your VST plugin folder
- Start your DAW, test the plugin
