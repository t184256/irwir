# irwir: an input wrapper

This is my first attempt at programming in Rust,
a generic input remapper program for Linux.

This initial version is pretty much a no-op.

Needs read access to input devices and write access to `/dev/uinput`.

Possible future goals:

* Multidevice remapping
* Automatically picking up the desired uinput device capabilities
* Multilayer remapping (e.g. to physical position first)
* A stack of major modes (with only the topmost transformation being active)
* A stack of minor modes (with all of the transformations being active)
* Precomputing all transformations into a single hashmap
* gluon scripting
