use druid::widget::prelude::*;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, ImageBuf, ExtEventSink, Target};
use druid::piet::{Image, InterpolationMode};
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use lz4_flex::block::decompress_size_prepended;
use image::{ImageBuffer, Rgba};
use std::sync::Arc;
use tokio::net::tcp::OwnedReadHalf;


struct RemoteDesktopWidget {
    image: Arc<Mutex<Option<ImageBuf>>>,
    updated: bool,  // track if new image data was received
}

impl Widget<()> for RemoteDesktopWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (), _env: &Env) {}
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}
    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {
        if self.updated {
            ctx.request_paint();
            self.updated = false;
        }
    }
    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &(), _env: &Env) -> Size {
        bc.max()
    }
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        if let Some(image_buf) = self.image.blocking_lock().as_ref() {
            let size = ctx.size();
            // if let Ok(image) = ctx.make_image(image_buf.size().width, image_buf.size().height, image_buf.raw_pixels(), druid::piet::ImageFormat::RgbaPremul) {
            //     ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
            // }

            if let Ok(image) = ctx.make_image(
                image_buf.size().width as usize,   // Cast width to usize
                image_buf.size().height as usize,  // Cast height to usize
                image_buf.raw_pixels(),
                druid::piet::ImageFormat::RgbaPremul
            ) {
                ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
            }
            
        }
    }
}

async fn receive_screen_data(mut stream: OwnedReadHalf, image: Arc<Mutex<Option<ImageBuf>>>, event_sink: ExtEventSink) -> Result<(), Box<dyn std::error::Error>> {
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

        *image.lock().await = Some(image_buf);
        
        // Safely submit the update event
        if let Err(e) = event_sink.submit_command(druid::Selector::new("UPDATE"), (), Target::Global) {
            eprintln!("Failed to submit update command: {}", e);
        }
    }
}

pub async fn render_screen(mut stream: OwnedReadHalf) -> Result<(), PlatformError> {
    let image = Arc::new(Mutex::new(None));
    let image_for_widget = image.clone();

    let window = WindowDesc::new(move || RemoteDesktopWidget { 
            image: image_for_widget.clone(), 
            updated: false,
        })
        .title("Remote Desktop Client")
        .window_size((1920.0, 1080.0));

    let launcher = AppLauncher::with_window(window);
    let event_sink = launcher.get_external_handle();

    let image_for_receiver = image.clone();
    tokio::spawn(async move {
        if let Err(e) = receive_screen_data(stream, image_for_receiver, event_sink).await {
            eprintln!("Error receiving screen data: {}", e);
        }
    });

    launcher.launch(())?;

    Ok(())
}
