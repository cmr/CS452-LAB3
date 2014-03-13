#[crate_id = "polyhedron"];
#[no_uv];

extern crate native;

extern crate gl;
extern crate hgl;
extern crate glfw = "glfw-rs";
extern crate cgmath;

use gl::types::{GLfloat, GLuint};
use std::mem::size_of;

use cgmath::matrix::Mat4;
use cgmath::array::Array;
use gl::types::GLint;
use hgl::{Shader, Program, Triangles, Vbo, Vao, Ebo};

static VERTEX_SHADER: &'static str = "\
#version 140

uniform mat4 xform;
uniform vec3 rot;
uniform float scale;
uniform int draw_black;

in vec3 position;
in vec3 color;

out vec3 oColor;

void main() {
    vec4 pos = vec4(position, 1.0);
    vec3 rrot = radians(rot);
    vec3 s = sin(rrot);
    vec3 c = cos(rrot);
    float scale_ = scale;

    mat4 rxm = mat4( 1, 0, 0, 0,
                     0, c.x, s.x, 0,
                     0, -s.x, c.x, 0,
                     0, 0, 0, 1);

    mat4 rym = mat4( c.y, 0, -s.y, 0,
                     0, 1, 0, 0,
                     s.y, 0, c.y, 0,
                     0, 0, 0, 1);

    mat4 rzm = mat4( c.z, s.z, 0, 0,
                     -s.z, c.z, 0, 0,
                     0, 0, 1, 0,
                     0, 0, 0, 1);

    if (draw_black == 1) {
        scale_ += 0.0001;
    }

    mat4 scale = mat4(scale_, 0, 0, 0,
                      0, scale_, 0, 0,
                      0, 0, scale_, 0,
                      0, 0, 0, 1);

    //gl_Position = trans * scale * rotx * roty * pos;
    gl_Position = xform * rxm * rym * rzm * scale * pos;
    oColor = color;
}";

static FRAGMENT_SHADER: &'static str = "\
#version 140
out vec4 out_color;
in vec3 oColor;
uniform int draw_black;

void main() {
    if (draw_black == 1) {
        out_color = vec4(vec3(0), 1);
    } else {
        out_color = vec4(oColor, 1);
    }
}";

static VERTICES: &'static [GLfloat] = &[
    -0.5,  0.5,  0.5, 0.0,  0.0, 0.0, // 0
     0.5,  0.5,  0.5, 1.0,  1.0, 1.0, // 1
    -0.5, -0.5,  0.5, 1.0,  0.0, 0.0, // 2
     0.5, -0.5,  0.5, 0.0,  1.0, 0.0, // 3

     0.0,  0.0,  0.0, 0.0,  0.0, 1.0, // 4

    -0.5,  0.5, -0.5, 1.0,  1.0, 0.0, // 5
     0.5,  0.5, -0.5, 1.0,  0.0, 1.0, // 6
    -0.5, -0.5, -0.5, 0.0,  1.0, 1.0, // 7
     0.5, -0.5, -0.5, 0.5, 0.75, 0.3, // 8
];

static INDICES: &'static [GLuint] = &[
    // top face
    3, 2, 1,
    1, 2, 0,

    // top pointy bit
    1, 0, 4,
    3, 1, 4,
    2, 3, 4,
    0, 2, 4,

    // bottom face
    7, 8, 6,
    7, 6, 5,

    // bottom point bit

    5, 6, 4,
    6, 8, 4,
    8, 7, 4,
    7, 5, 4,
];

#[start]
fn main(argc: int, argv: **u8) -> int {
    native::start(argc, argv, proc() {
        glfw::set_error_callback(box glfw::LogErrorHandler);
        glfw::start(proc() {
            glfw::window_hint::samples(4);
            glfw::window_hint::context_version(3, 2);
            glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);
            let window = glfw::Window::create(800, 600, "Lab 3", glfw::Windowed).unwrap();

            let (mut rx, mut ry, mut rz) = (0.0, 0.0, 0.0);
            let mut scale = 1.0;

            window.set_mouse_button_polling(true);
            window.set_key_polling(true);
            window.make_context_current();
            gl::load_with(glfw::get_proc_address);

            gl::Viewport(0, 0, 800, 600);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            let vao = Vao::new(); vao.bind();

            let program = Program::link([
                Shader::compile(VERTEX_SHADER, hgl::VertexShader).unwrap(),
                Shader::compile(FRAGMENT_SHADER, hgl::FragmentShader).unwrap()
            ]).unwrap();

            program.bind_frag(0, "out_color");
            program.bind();

            let _vbo = Vbo::from_data(VERTICES, hgl::buffer::StaticDraw);
            let _ebo = Ebo::from_indices(INDICES);

            vao.enable_attrib(&program, "position", gl::FLOAT, 3, 6*std::mem::size_of::<GLfloat>() as GLint, 0);
            vao.enable_attrib(&program, "color", gl::FLOAT, 3, 6*std::mem::size_of::<GLfloat>() as GLint, 3*std::mem::size_of::<GLfloat>());

            let tpos = program.uniform("xform");
            let rpos = program.uniform("rot");
            let spos = program.uniform("scale");

            let black = program.uniform("draw_black");

            let mut xform: Mat4<f32> = Mat4::identity();

            while !window.should_close() {
                glfw::poll_events();

                for (_, event) in window.flush_events() {
                    match event {
                        glfw::KeyEvent(glfw::KeyW, _, _, _) => {
                            xform.w.y += 0.1;
                        },
                        glfw::KeyEvent(glfw::KeyS, _, _, _) => {
                            xform.w.y -= 0.1;
                        },
                        glfw::KeyEvent(glfw::KeyD, _, _, _) => {
                            xform.w.x += 0.1;
                        },
                        glfw::KeyEvent(glfw::KeyA, _, _, _) => {
                            xform.w.x -= 0.1;
                        },
                        glfw::KeyEvent(glfw::KeyI, _, _, _) => {
                            rx += 1.0;
                            debug!("rx is {}", ry)
                        },
                        glfw::KeyEvent(glfw::KeyK, _, _, _) => {
                            rx -= 1.0;
                            debug!("rx is {}", ry)
                        },
                        glfw::KeyEvent(glfw::KeyJ, _, _, _) => {
                            ry += 1.0;
                            debug!("ry is {}", rx)
                        },
                        glfw::KeyEvent(glfw::KeyL, _, _, _) => {
                            ry -= 1.0;
                            debug!("ry is {}", rx)
                        },
                        glfw::KeyEvent(glfw::KeyU, _, _, _) => {
                            rz += 1.0;
                            debug!("rz is {}", rx)
                        },
                        glfw::KeyEvent(glfw::KeyO, _, _, _) => {
                            rz -= 1.0;
                            debug!("rz is {}", rx)
                        },
                        glfw::KeyEvent(glfw::KeyE, _, _, _) => {
                            scale += 0.01;
                        },
                        glfw::KeyEvent(glfw::KeyQ, _, _, _) => {
                            scale -= 0.01;
                        },
                        _ => { }
                    }
                }

                gl::Uniform3f(rpos, rx, ry, rz);
                gl::Uniform1f(spos, scale);

                unsafe {
                    gl::UniformMatrix4fv(tpos, 1, gl::FALSE, xform.as_slice().as_ptr() as *GLfloat);
                }

                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                vao.draw_elements(hgl::Triangles, 0, 36);
                gl::Uniform1i(black, 1);
                vao.draw_elements(hgl::LineStrip, 0, 36);
                gl::Uniform1i(black, 0);

                window.swap_buffers();
            }
        });
    });
    0
}

