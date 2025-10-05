use crate::models::display_picture::DisplayPicture;
use crate::sqlite::Sqlite;
use image::imageops::FilterType;
use msnp11_sdk::Client;
use rfd::FileHandle;
use std::io::{Cursor, ErrorKind};
use std::sync::Arc;

pub async fn pick_display_picture(
    picture_future: impl Future<Output = Option<FileHandle>>,
    email: Arc<String>,
    client: Arc<Client>,
    sqlite: Sqlite,
) -> Result<DisplayPicture, Box<dyn std::error::Error + Sync + Send>> {
    let picture = picture_future.await.ok_or(std::io::Error::new(
        ErrorKind::NotFound,
        "Display picture not found",
    ))?;

    let mut bytes = Vec::new();
    let picture = image::open(picture.path())?;
    picture
        .resize_to_fill(200, 200, FilterType::Triangle)
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;

    let hash = client.set_display_picture(bytes.clone()).await?;
    let _ = sqlite.insert_display_picture(&bytes, &hash);
    let _ = sqlite.update_user_display_picture(&email, &hash);

    let bytes = bytes.into_boxed_slice();
    Ok(DisplayPicture {
        data: Arc::from(bytes),
        hash: Arc::new(hash),
    })
}
