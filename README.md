# wgpu lite wrapper

Hobby project to test the Rust programming language and WebGPU through wgpu.

Used technologies: Rust, winit, WebAssembly, WebGPU, WebGL.

## Some convention choices

- coordinate system is left-handed
- triangle front face is counter clock wise

## How to launch

    $ cargo run

There are also some examples in the `examples` directory:

    $ cargo run --example simple_triangle

For the web version, you must be sure you can compile to the WebAssembly target first:

    $ rustup target add wasm32-unknown-unknown

You can then launch the command:

    $ ./run-web.sh

... then launch a browser with the displayed URL.

# References

I heavily read and used the [Learn WGPU tutorial](https://sotrh.github.io/learn-wgpu).

Mains references links are:
- https://github.com/gfx-rs/wgpu
- https://www.w3.org/TR/webgpu
- https://www.w3.org/TR/WGSL

Having a look to some [wasm-compatible examples](https://github.com/gfx-rs/wgpu/tree/master/wgpu/examples)
did helped a lot too. 

## License

MIT License

Copyright (c) 2021, 2022, 2024, 2025 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.