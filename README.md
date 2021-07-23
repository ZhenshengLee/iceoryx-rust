# iceoryx-rs-c

<p align="center">
<img src="https://user-images.githubusercontent.com/8661268/114321508-64a6b000-9b1b-11eb-95ef-b84c91387cff.png" width="50%">
</p>

Rust wrapper for the [iceoryx](https://github.com/eclipse-iceoryx/iceoryx) IPC middleware based on rust bindgen.

## patch with iceoryx c binding

Because of this [known issue with iceoryx_binding_c](https://github.com/eclipse-iceoryx/iceoryx/issues/879)

You must build with this version of [iceoryx](https://github.com/ZhenshengLee/iceoryx/tree/rust-c-binding), which changes the `binding_c` to full cpp symbol.

By default, iceoryx will be installed in `/usr/local`, and you can install this version in `/opt/iceoryx/iceoryx` with following command.

```sh
git clone https://github.com/ZhenshengLee/iceoryx.git
cd ./iceoryx
git checkout rust-c-binding
cmake -Bbuild -Hiceoryx_meta -DBUILD_ALL=1 -DBUILD_SHARED_LIBS=ON -DCMAKE_INSTALL_PREFIX=/opt/iceoryx/iceoryx/
cmake --build build  -j10
sudo cmake --build build --target install
# -DBUILD_SHARED_LIBS=ON is needed, perhaps due to bugs with bindgen
```

in `build.rs`, rust will try to find iceoryx libs in `/opt/iceoryx/iceoryx/lib`, add any path in `build.rs` if needed.

## build

```sh
cargo build
# or with release
cargo build --release
```

## run the examples

### icedelivery

```sh
# t1
iox-roudi
# t2
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/opt/iceoryx/iceoryx/lib
./target/debug/ice_rs_publisher
# t3
iox-c-subscriber
```

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
