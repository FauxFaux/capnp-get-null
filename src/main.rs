extern crate capnp;

mod gener {
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/foo_capnp.rs"));
}

pub fn write(msg: &str) -> Vec<u8> {
    let mut message = capnp::message::Builder::new_default();
    {
        let mut output = message.init_root::<gener::foo::Builder>();
        output.set_with(msg);
    }
    let mut save = Vec::new();
    capnp::serialize::write_message(&mut save, &message).expect("saving");
    save
}

pub fn read(val: &[u8]) -> (String, String) {
    let message = capnp::serialize::read_message(
        &mut std::io::Cursor::new(val),
        capnp::message::ReaderOptions::new(),
    ).expect("opening");
    let input = message.get_root::<gener::foo::Reader>().expect("get root");

    (
        input.get_with().expect("with").to_string(),
        input.get_without().expect("without").to_string(),
    )
}

fn main() {
    assert_eq!(
        ("yellow".to_string(), "".to_string()),
        read(&write("yellow"))
    )
}
