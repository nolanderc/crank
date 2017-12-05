
use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_device_gl;
use gfx_device_gl::Factory;

use glutin;
use gfx_window_glutin;

use window;

pub mod frame;
use self::frame::RenderFrame;

mod shape;

pub mod camera;


gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<window::ColorFormat> = "Target0",
    }
}



type R = gfx_device_gl::Resources;
type C = gfx_device_gl::CommandBuffer;

pub struct Renderer {
    device: gfx_device_gl::Device,
    factory: Factory,
    color_view: gfx::handle::RenderTargetView<R, window::ColorFormat>,
    depth_view: gfx::handle::DepthStencilView<R, window::DepthFormat>,

    pso: gfx::pso::PipelineState<R, pipe::Meta>,
    encoder: gfx::Encoder<R, C>
}

impl Renderer {
    pub fn new(device: gfx_device_gl::Device,
               mut factory: Factory,
               color_view: gfx::handle::RenderTargetView<R, window::ColorFormat>,
               depth_view: gfx::handle::DepthStencilView<R, window::DepthFormat>) -> Self {

        let pso = Renderer::create_pipeline(&mut factory);

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        Renderer {
            device,
            factory,
            color_view,
            depth_view,

            pso,
            encoder
        }
    }


    /// Renders a frame
    pub fn draw(&mut self, frame: RenderFrame) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        self.encoder.clear(&self.color_view, [0.0, 0.0, 0.0, 0.0]); //clear the framebuffer with a color(color needs to be an array of 4 f32s, RGBa)
        for camera_state in frame.cameras.into_iter() {
            vertices.clear();
            indices.clear();

            for index in camera_state.get_shapes() {
                let shape = &frame.shapes[index as usize];
                let start_index = vertices.len() as u32;
                let new_indices = shape.indices.iter().map(|index| index + start_index);

                indices.extend(new_indices);
                vertices.extend(shape.vertices.iter());
            }


            // No need to render nothing
            if vertices.len() > 0 {
                // Camera Matrix
                let transform: Transform = Transform { transform: camera_state.get_transform() };

                let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&vertices, indices.as_slice());
                let transform_buffer = self.factory.create_constant_buffer(1);
                let data = pipe::Data {
                    vbuf: vertex_buffer,
                    transform: transform_buffer,
                    out: self.color_view.clone(),
                };

                self.encoder.update_buffer(&data.transform, &[transform], 0).unwrap(); //update buffers
                self.encoder.draw(&slice, &self.pso, &data); // draw commands with buffer data and attached pso
            }

        }

        self.encoder.flush(&mut self.device); // execute draw commands
    }


    /// Creates a new pipeline object
    fn create_pipeline(factory: &mut Factory) -> gfx::pso::PipelineState<R, pipe::Meta> {
        let set = factory.create_shader_set(
            include_bytes!("shaders/shader.vert"),
            include_bytes!("shaders/shader.frag")
        ).unwrap();

        factory.create_pipeline_state(
            &set,
            gfx::Primitive::TriangleList,
            gfx::state::Rasterizer{
                samples: Some(gfx::state::MultiSample{}),
                ..gfx::state::Rasterizer::new_fill()
            },
            pipe::new()
        ).unwrap()
    }

    /// Create a new frame to render to
    pub fn get_new_frame(&mut self) -> RenderFrame {
        RenderFrame::new()
    }


    /// Clears all leftover rendering data
    pub fn clean(&mut self) {
        self.device.cleanup();
    }


    /// Sets the viewport
    pub fn set_viewport(&mut self, window: &glutin::GlWindow) {
        gfx_window_glutin::update_views(window, &mut self.color_view, &mut self.depth_view);
    }
}
