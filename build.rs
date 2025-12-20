use std::io::Result;

fn main() -> Result<()> {
    tonic_prost_build::configure()
        .build_server(false)
        .build_client(false)
        .compile_protos(&["src/proto/cloudevents.proto"], &["src"])?;

    Ok(())
}
