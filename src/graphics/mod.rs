pub mod mesh;
pub mod renderer;
pub mod shader;
pub mod test;
pub mod texture;

use gl;
use glutin;
use glutin::GlContext;
use std::ffi::CString;
use std::str;

////////////////////////////////////////////////////////////////////////////////

fn c_str_ptr(s: &str) -> *const i8 {
    CString::new(s).unwrap().as_ptr()
}

////////////////////////////////////////////////////////////////////////////////

pub trait Render {
    fn render(&mut self, context: &mut renderer::RenderingContext);
}

////////////////////////////////////////////////////////////////////////////////

fn clear_screen() {
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Display {
    gl_window: glutin::GlWindow,
}

impl Display {
    pub fn new(window: glutin::WindowBuilder, context: glutin::ContextBuilder,
               events_loop: &glutin::EventsLoop) -> Option<Display> {
        use glutin::GlContext;

        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe { gl_window.make_current() }.unwrap();

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        // TODO: gl::CullFace(gl::BACK);

        Some(Display {
            gl_window,
        })
    }

    pub fn finish_frame(&self) {
        self.gl_window.swap_buffers().unwrap();
    }

    pub fn gl_window(&self) -> &glutin::GlWindow {
        &self.gl_window
    }
}

