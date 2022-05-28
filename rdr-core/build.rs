use std::fs;


fn main() {
    protobuf_codegen::Codegen::new()
        .includes(&["src/message/proto"])
        .inputs(fs::read_dir("src/message/proto").unwrap()
            .filter_map(|rs| rs.ok().map(|entry| entry.path()))
            .filter(|path| matches!(
                path.extension(), Some(ext) if ext == "proto"))
        )
        .out_dir("src/message")
        .customize(protobuf_codegen::Customize::default()
            .tokio_bytes(true))
        .run_from_script();
}