use std::fs;

use directories::BaseDirs;

pub async fn ensure_app_dirs() -> std::io::Result<String> {
    let app_name = "sqlx";

    let base_dirs = BaseDirs::new().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine base directories",
        )
    })?;

    let app_config = base_dirs.config_dir().join(app_name);
    fs::create_dir_all(&app_config)?;

    Ok(app_config.to_string_lossy().to_string())
}
