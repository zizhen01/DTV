use std::io::Result;

fn main() -> Result<()> {
    // We no longer manually create the output directory in src/
    // prost_build will automatically use the OUT_DIR environment variable
    // which is the standard place for build artifacts.

    let mut config = prost_build::Config::new();
    
    // Customize code generation if needed, otherwise defaults are usually fine.
    // If you need specific type attributes (like serde derive), add them here.
    
    config.compile_protos(
            &["src/platforms/douyin/danmu/douyin.proto"],
            &["src/platforms/douyin/danmu/"],
        )
        .expect("Failed to compile douyin danmu protos");

    println!("cargo:rerun-if-changed=src/platforms/douyin/danmu/douyin.proto");
    Ok(())
}