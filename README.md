# iceoryx-rs-c

<p align="center">
<img src="https://user-images.githubusercontent.com/8661268/114321508-64a6b000-9b1b-11eb-95ef-b84c91387cff.png" width="50%">
</p>

Rust wrapper for the [iceoryx](https://github.com/eclipse-iceoryx/iceoryx) IPC middleware based on rust bindgen.

## install iceoryx

To avoid version confusion, this repo relys on header files from a installed or predefined directory.

Please install iceoryx first.

Because of this [known issue with iceoryx_binding_c](https://github.com/eclipse-iceoryx/iceoryx/issues/879)

You must install my version of [iceoryx](https://github.com/ZhenshengLee/iceoryx/tree/release_1.0), which changes the `binding_c` to full cpp symbol.

## build

```sh
cargo build
# or with release
cargo build --release
```

## run the examples

This repo is currently under development as `iceoryx-binding-c` is not ready to use.

| examples              | Status                             |
|-----------------------|------------------------------------|
| icedelivery           | :heavy_check_mark:                 |
| callback              | :x:                                |
| waitset               | :x:                                |
| :grey_question:       | :grey_question:                    |

## todo

| todos                 | Status                             |
|-----------------------|------------------------------------|
| create safe api       | :x:                                |
| block and allow       | :x:                                |
| :grey_question:       | :grey_question:                    |

## related work

See [iceoryx-rs](https://github.com/elBoberido/iceoryx-rs) created by iceoryx team with a higher abstraction of rust api.

Which is undone for now.
