# irwir: an input wrapper

This is my first attempt at programming in Rust,
a generic input remapper program for Linux.


## How it works

Event from a real device -> Tag (String) -> gluon function -> Action.

See `config.toml` for an idea of what can be done right now.

Needs read access to input devices and write access to `/dev/uinput`.


## Goals

* Multidevice remapping
* Automatically picking up the desired uinput device capabilities
* Multilayer remapping (e.g. to physical position first)
* A stack of major modes (with only the topmost transformation being active)
* A stack of minor modes (with all of the transformations being active)
* Precomputing all active transformations into a single hashmap
