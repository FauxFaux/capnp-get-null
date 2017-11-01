extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("foo.capnp")
        .run()
        .expect("schema compiler command");
}
