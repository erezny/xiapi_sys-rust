xiapi_sys-rust
============

A [https://github.com/crabtw/rust-bindgen][bindgen] binding for Ximea xiApi.

Requirements
------------

* OSX: [issue 1][].

* xiAPI: http://www.ximea.com/support/documents/4

* clang 3.4 and up

Note: The libclang.so has to be statically linked with LLVM or you will
encounter [issue 89][]. You can also use LD_PRELOAD=/path/to/libclang.so to
workaround the problem.

Building
--------

    $ cargo build

Note: This links with Apple's version of libclang on OS X by default. This can be changed by setting the LIBCLANG_PATH environment variable.

If you are running the command line tool you will also need to append this
path to your DYLD_LIBRARY_PATH environment variable, which you might already have set if you have installed the Rust compiler outside of standard /usr/local path.

The default path on OS X is:

    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/

Or if you only have Xcode Command Line Tools installed:

    export DYLD_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib

Examples
--------

see examples/

TODO
----

* use pkg_config or similar for xiAPI location.
