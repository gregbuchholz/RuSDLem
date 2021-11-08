# RuSDLem - A minimal Rust program using SDL2 to target Emscripten and the web 

There are a number of tutorials out there with the goal of showing how to use
Rust and the [SDL2](https://github.com/Rust-SDL2/rust-sdl2) graphics library to
build program with a web target.  Unfortunately, most of them have developed
bit rot over the years.  You should use those as the tutorial to get a sense
for how things should work.  This example shows a minimal working solution as
of November of 2021.  You can see the example in action at:

https://gregbuchholz.github.io/

You will need [Emscipten](https://emscripten.org/) installed and you path
updated to point to the installation (Unix-like: `source ~/emsdk/emsdk_env.sh`
or on Windows: `emsdk_env.bat`).

Grab this repository:

    git clone https://github.com/gregbuchholz/RuSDLem

One or both of the Emscripten targets will need to be added to your Rust setup:

    rustup target add asmjs-unknown-emscripten
    rustup target add wasm32-unknown-emscripten

...then verify things work locally as a desktop app with:
    
    cd rusdlem
    cargo run

...followed by...

    em++ -c gxx_personality_v0_stub.c

...in the `src/` directory to produce a stub `gxx_personality_v0_stub.o` in
`src/`.  The build the project with:

    cargo build --target=wasm32-unknown-emscripten --release

..and then run a web server with:

    emrun index-wasm.html

...or use the following if you are interested in asmjs...

    cargo build --target=asmjs-unknown-emscripten --release
    emrun index-asmjs.html

Enjoy!

===========

# Further Details

Here are a few more details about the various pieces that are necessary but
aren't described in other places. The `.cargo/config` file has lines like the
following:

    [target.asmjs-unknown-emscripten]
    rustflags = [
        "-C", "link-args=src/gxx_personality_v0_stub.o -sUSE_SDL=2 -s ASYNCIFY -s ALLOW_MEMORY_GROWTH=1",
    ]

The `ALLOW_MEMORY_GROWTH=1` isn't needed for this small example, but it does
become necessary if you start using larger bitmaps/render targets.  The
symptoms are mysterious memory allocation failure assertions reported by the
browser coming from the JS.  Apparently there is a 16 MiB heap limit hardcoded
into Emscripten that this alleviates.  With all the back buffers, etc, it
doesn't take much to reach this limit.

`ASYNCIFY` enables
[Ansyncify](https://emscripten.org/docs/porting/asyncify.html), which for this
example has strategic calls to `emscripten_sleep()` which enables the browser's
main loop to have control periodically and this means we don't have to deal
with `set_main_loop` and friends.  You'll also notice the:

    #[cfg(target_os = "emscripten")]
    let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY","1");

...before the `sdl2::init()` in the main.rs file.

The `gxx_personality_v0_stub.cpp` file is there to work around the issue
described at: https://stackoverflow.com/a/69198170
Without it, you'll probably get a message similar to the following:

    = note: error: undefined symbol: __gxx_personality_v0 (referenced by top-level compiled C/C++ code)
            warning: Link with `-s LLD_REPORT_UNDEFINED` to get more information on undefined symbols
            warning: To disable errors for undefined symbols use `-s ERROR_ON_UNDEFINED_SYMBOLS=0`
            warning: ___gxx_personality_v0 may need to be added to EXPORTED_FUNCTIONS if it arrives from a system library
            Error: Aborting compilation due to previous errors

If you don't include `--release` on the emscripten builds, you might wind up
with an error message:

    = note: emcc: warning: please replace -g4 with -gsource-map [-Wdeprecated]
            emcc: error: wasm2js does not support source maps yet (debug in wasm for now)

The versions that this was know to work with:

    $ rustc --version
    rustc 1.55.0 (c8dfcfe04 2021-09-06)

    $ emcc --version
    emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 2.0.31 (4fcbf0239ccca29771f9044c990b0d34fac6e2e7)
    Copyright (C) 2014 the Emscripten authors (see AUTHORS.txt)
    This is free and open source software under the MIT license.
    There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

