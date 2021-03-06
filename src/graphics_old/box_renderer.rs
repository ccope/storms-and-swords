use cgmath::{self, Vector2};
use gfx;
use gfx_device_gl;
use gfx_device_gl::{Resources};
use gfx::{Device, CommandQueue,FrameSync, GraphicsPoolExt,
          Surface, Swapchain, SwapchainExt, WindowExt};
use gfx::traits::DeviceExt;

use graphics::render_thread::{RenderPackage, RenderThread};
type ColorFormat = gfx::format::Rgba8;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


gfx_defines!{
    vertex BoxVertex {
        pos: [f32;2] = "a_Pos",
        color: [f32;3] = "color",
        rotation: f32 = "rotation",
    }

    constant Transform {
        prop: [[f32;4];4] = "u_prop",
    }

    pipeline BoxPipeLine {
        vbuf: gfx::VertexBuffer<BoxVertex> = (),
        perp: gfx::ConstantBuffer<Transform> = "Pro",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

#[derive(Clone)]
pub struct BoxRenderData {
    pub pos: Vector2<f32>,
    pub scale: Vector2<f32>,
    pub z_rotation: f32,
    pub color: [f32; 3],
}

pub struct BoxRenderer {
    pso: gfx::PipelineState<Resources, BoxPipeLine::Meta>,
    graphics_pool: gfx::GraphicsCommandPool<gfx_device_gl::Backend>,
}

impl BoxRenderer {
    
    pub fn new(device: &mut gfx_device_gl::Device, graphics_pool: gfx::GraphicsCommandPool<gfx_device_gl::Backend>) -> BoxRenderer {

        let pso = device.create_pipeline_simple(
            include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/box_shader.vs"
                )),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/box_shader.fs"
                )),
                BoxPipeLine::new(),
        ).unwrap();

        BoxRenderer {
            pso,
            graphics_pool
        }
        
    }

    pub fn render_boxes(&mut self, boxes_to_render: &Vec<BoxRenderData>, render_package: &mut RenderPackage, view: &gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>, rt : &mut RenderThread) {
        
        let mut vertex_info = vec![];
        let mut index_info : Vec<u16> = vec![];

      //  let mut graphics_pool = render_package.graphics_queue.create_graphics_pool(1);

        for box_to_render in boxes_to_render.iter() {
            vertex_info.extend(&[
                BoxVertex{pos: [box_to_render.pos.x + (-0.5f32 * box_to_render.scale.x), box_to_render.pos.y  + (-0.5f32 * box_to_render.scale.y)], color: box_to_render.color, rotation: box_to_render.z_rotation},//top left
                BoxVertex{pos: [box_to_render.pos.x + ( 0.5f32 * box_to_render.scale.x), box_to_render.pos.y  + (-0.5f32 * box_to_render.scale.y)], color: box_to_render.color, rotation: box_to_render.z_rotation},//top right
                BoxVertex{pos: [box_to_render.pos.x + (-0.5f32 * box_to_render.scale.x), box_to_render.pos.y  + ( 0.5f32 * box_to_render.scale.y)], color: box_to_render.color, rotation: box_to_render.z_rotation},//bottom left
                BoxVertex{pos: [box_to_render.pos.x + ( 0.5f32 * box_to_render.scale.x), box_to_render.pos.y  + ( 0.5f32 * box_to_render.scale.y)], color: box_to_render.color, rotation: box_to_render.z_rotation}//bottom right
                ]
            );
        }

        for i in 0..boxes_to_render.len() {
            let i = i as u16;
            index_info.extend(&[0 + (i * 4), 1 + (i * 4), 2 + (i * 4),//top left triangle
                                2 + (i * 4), 1 + (i * 4), 3 + (i * 4)]);//bottom right triangle
        }
        
        let (vertex_buffer, index_buffer) = render_package.device.create_vertex_buffer_with_slice(&vertex_info, &*index_info);

        let t = Transform{
            prop: rt.use_matrix
        };

        let constant_buffer = render_package.device.create_constant_buffer(1);

        let box_data = BoxPipeLine::Data {
            vbuf: vertex_buffer.clone(),
            perp: constant_buffer,
            out: view.clone(),
        };

        {
            let mut box_encoder = self.graphics_pool.acquire_graphics_encoder();
            let _ = box_encoder.update_buffer(&box_data.perp, &[t], 0);
            box_encoder.clear(&box_data.out, BLACK);
            box_encoder.draw(&index_buffer, &self.pso, &box_data);
            let _ = box_encoder.synced_flush(render_package.graphics_queue, &[&render_package.frame_semaphore], &[&render_package.draw_semaphore], Some(&render_package.frame_fence)).expect("could not flush encoder");
        }
        self.graphics_pool.reset();
    }
}