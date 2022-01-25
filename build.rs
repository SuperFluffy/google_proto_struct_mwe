use std::io;

fn main() -> io::Result<()> {
    let builder = tonic_build::configure()
        .compile_well_known_types(true);

    builder.compile(&["./protos/myproto.proto",], &["./protos"])?;
    Ok(())
}
