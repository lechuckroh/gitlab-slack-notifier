use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Body {
    object_kind: String,
}
#[derive(Deserialize, Debug)]
struct Payload {
    body: Body,
}

fn read_payload_from_file<P: AsRef<Path>>(path: P) -> Result<Payload, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let payload = serde_json::from_reader(reader)?;
    Ok(payload)
}

#[test]
fn works() {
    let payload = read_payload_from_file("../events/mr-open.json").unwrap();
    println!("payload: {:#?}", payload);
    assert!(true)
}
