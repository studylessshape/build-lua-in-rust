use std::{env, fs::File};

use lua_rs::{parse, vm};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     println!("Usage: {} script", args[0]);
    //     return;
    // }

    // let file = File::open(&args[1]).unwrap();
    let file = File::open("test_lua/assign.lua").unwrap();

    let proto = parse::ParseProto::load(file);
    if let Err(err) = proto {
        panic!("{}", err)
    }
    let proto = proto.unwrap();
    vm::ExeState::new().execute(&proto);
}
