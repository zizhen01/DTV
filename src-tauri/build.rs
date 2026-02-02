use std::io::Result;

fn main() -> Result<()> {
    tauri_build::build();

    Ok(())
}
