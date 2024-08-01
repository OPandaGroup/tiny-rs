use tinify::async_bin::Tinify;

pub async fn process_images(tinify: Tinify, path: Vec<String>) -> anyhow::Result<()> {
    for p in path {
        let mut new_file_name = Vec::new();
        let mut optimized_string_is_added = false;

        p.split(".").for_each(|i| {
            if !i.is_empty() && !optimized_string_is_added {
                new_file_name.push(format!("{i}-optimized"));
                optimized_string_is_added = true;
            } else {
                new_file_name.push(i.to_string());
            }
        });

        let new_file_name = new_file_name.join(".");

        tinify
            .get_async_client()?
            .from_file(p.clone())
            .await?
            .to_file(new_file_name.clone())
            .await?;
    }
    Ok(())
}
