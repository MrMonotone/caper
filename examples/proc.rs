extern crate time;

#[macro_use]
extern crate caper;

use caper::utils::load_wavefront;
use caper::renderer::{ RenderItem, Transform };

fn main() {
    // generate the instance positions 
    let transforms = (0 .. 200)
        .map(|i| {
            Transform {
                pos: ((i as f32 % 10f32) * 2f32, 0.0f32, (i as f32 / 10f32) * 2f32),
                rot: (0f32, 0f32, 0f32, 1f32),
                scale: (1f32, 1f32, 1f32)
            }
        })
    .collect::<Vec<_>>();

    // create a vector of render items
    let mut render_items = vec![
        RenderItem {
            vertices: load_wavefront(include_bytes!("assets/sphere.obj")),
            shader_index: 1,
            instance_transforms: transforms
        }
    ];

    game_loop! {
        // pass the items to be rendered
        render_items,
        // define a block for update
        { 
            // update some items
            let update_time = time::precise_time_s();

            render_items[0].instance_transforms =
                render_items[0].instance_transforms.iter().map(|t| {
                    Transform {
                        pos: (t.pos.0,
                              ((t.pos.0 / 5f32).sin() *
                               (t.pos.2 / 5f32).cos() *
                               update_time.sin() as f32) * 2f32,
                               t.pos.2),
                        rot: (0f32, 0f32, 0f32, 1f32),
                        scale: (update_time.sin() as f32,
                               update_time.sin() as f32,
                               update_time.sin() as f32)
                    }
                }).collect::<Vec<_>>();
        }
    }
}