use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //intellij does not easily find types when not adding code to src
    //tonic_build::compile_protos("proto/Flight.proto")?;
    //tonic_build::compile_protos("proto/FlightSql.proto")?;

    let out_dir = "./src";

    let path = Path::new("./proto/Flight.proto");
    if path.exists() {
        println!("cargo:rerun-if-changed=./proto/Flight.proto");
        tonic_build::configure()
            .out_dir(out_dir)
            .compile(&["proto/Flight.proto"], &["proto"])?;
    }

    let path = Path::new("./proto/FlightSql.proto");
    if path.exists() {
        println!("cargo:rerun-if-changed=./proto/FlightSql.proto");
        tonic_build::configure()
            .out_dir(out_dir)
            .compile(&["proto/FlightSql.proto"], &["proto"])?;
    }

    Ok(())
}
