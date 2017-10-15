pub mod renderer;

pub trait Render {
    fn render(&mut self, context: &mut renderer::RenderingContext);
}
