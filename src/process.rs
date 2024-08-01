use tinify::async_bin::Tinify;

pub async fn process_images(tinify: Tinify, path: Vec<String>) -> anyhow::Result<()> {
    for p in path {
        let mut iter = p.split(".");
        let front = iter.next().unwrap_or("new");
        let extention = iter.last().unwrap();
        let new_file_name = format!("{}-optimized.{}", front, extention);
        tinify
            .get_async_client()?
            .from_file(p.clone())
            .await?
            .to_file(new_file_name.clone())
            .await?;
    }
    Ok(())
}
