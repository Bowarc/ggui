mod color;
mod draw_param;
mod render_log;
mod render_request;

pub use color::Color;
pub use draw_param::DrawParam;
pub use render_log::RenderLog;
pub use render_request::{RenderRequest, RenderRequestBit};

// Sprite rendering is not done atm
pub fn render(
    ctx: &mut ggez::Context,
    render_request: &mut RenderRequest,
    canvas: &mut ggez::graphics::Canvas,
) -> ggez::GameResult<RenderLog> {
    use ggez::graphics::Drawable as _;

    let mut render_log = RenderLog::new();

    for (bit, dp) in (render_request).iter() {
        match bit {
            RenderRequestBit::Image(image) => {
                let Some(dimensions) = image.dimensions(ctx) else {
                    error!("Could not query the size of the image");
                    continue;
                };
                canvas.draw(image, dp.to_ggez_scaled(dimensions.size()));

                render_log.on_image();
                render_log.on_draw_call();
            }
            RenderRequestBit::Mesh(mesh) => {
                canvas.draw(mesh, dp.to_ggez_unscaled());
                render_log.on_mesh();
                render_log.on_draw_call();
            }
            RenderRequestBit::MeshBuilder(mesh_buuilder) => {
                canvas.draw(
                    &ggez::graphics::Mesh::from_data(ctx, mesh_buuilder.build()),
                    dp.to_ggez_unscaled(),
                );
                render_log.on_mesh();
                render_log.on_draw_call();
            }
            RenderRequestBit::Text(text) => {
                canvas.draw(text, dp.to_ggez_unscaled());
                render_log.on_text();
                render_log.on_draw_call();
            }
        }
    }

    render_request.clear();

    Ok(render_log)
}
