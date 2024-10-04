use druid::widget::prelude::*;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, ImageBuf};
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use lz4_flex::block::decompress_size_prepended;
use image::{ImageBuffer, Rgba};
use std::sync::{Arc, Mutex};

struct RemoteDesktopWidget {
    image: Arc<Mutex<Option<ImageBuf>>>,
}

impl Widget<()> for RemoteDesktopWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (), _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &(), _env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        if let Some(image) = self.image.lock().unwrap().as_ref() {
            ctx.draw_image(image, ctx.size().to_rect(), InterpolationMode::Bilinear);
        }
    }
}

async fn receive_screen_data(stream: &mut TcpStream, image: Arc<Mutex<Option<ImageBuf>>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut size_buffer = [0u8; 4];
    loop {
        stream.read_exact(&mut size_buffer).await?;
        let size = u32::from_le_bytes(size_buffer) as usize;

        let mut compressed_data = vec![0u8; size];
        stream.read_exact(&mut compressed_data).await?;

        let decompressed_data = decompress_size_prepended(&compressed_data)?;
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::from_raw(1920, 1080, decompressed_data)
            .ok_or("Failed to create image buffer")?;

        let image_buf = ImageBuf::from_raw(
            img.into_raw(),
            druid::piet::ImageFormat::RgbaPremul,
            1920,
            1080,
        );

        *image.lock().unwrap() = Some(image_buf);
    }
}

pub async fn render_screen(stream: TcpStream)  -> Result<(), PlatformError> {
    let image = Arc::new(Mutex::new(None));
    let image_clone = image.clone();

    if let Err(e) = receive_screen_data(&mut stream, image_clone).await {
        eprintln!("Error receiving screen data: {}", e);
    }

    let window = WindowDesc::new(RemoteDesktopWidget { image })
        .title("Remote Desktop Client")
        .window_size((1920.0, 1080.0));

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(())?;

    Ok(())
}