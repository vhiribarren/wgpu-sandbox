/*
MIT License

Copyright (c) 2021, 2022 Vincent Hiribarren

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
*/

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct FragmentInput {
    [[location(0)]] color: vec3<f32>;
    [[builtin(position)]] position: vec4<f32>;
};


// [[group(0), binding(0)]]
// var<uniform> transform: mat4x4<f32>;


[[block]]
struct TransformUniform {
    mat: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> transform: TransformUniform;


[[stage(vertex)]]
fn vtx_main(vtx_in: VertexInput) -> FragmentInput {
    var out: FragmentInput;
    out.color = vtx_in.color;
    out.position = transform.mat * vec4<f32>(vtx_in.position, 1.0);
    return out;
}

[[stage(fragment)]]
fn frg_main(frg_in: FragmentInput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(frg_in.color, 1.0);
}