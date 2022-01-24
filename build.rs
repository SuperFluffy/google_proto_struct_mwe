use std::io;

fn main() -> io::Result<()> {
    let builder = tonic_build::configure()
        .compile_well_known_types(true);

    let mut prost_config = prost_build::Config::new();
    prost_config
        .compile_well_known_types()
        .btree_map(&["."]);

    builder.compile_with_config(prost_config, &["./protos/myproto.proto",], &["./protos"])?;
    Ok(())
}
