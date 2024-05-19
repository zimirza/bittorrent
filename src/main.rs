use std::{collections::HashMap, sync::Arc};

#[derive(Debug)]
enum Bencode {
    Integer(u64),
    String(Arc<str>),
    List(&'static [Bencode]),
    Dictionary(HashMap<Arc<str>, Bencode>),
}

impl Bencode {
    fn encode(&self) -> &[u8] {
        match self {
            Bencode::Integer(i) => {
                todo!()
            }
            Bencode::String(s) => {
                todo!()
            }
            Bencode::List(l) => {
                todo!()
            }
            Bencode::Dictionary(e) => {
                todo!()
            }
        }
    }

    fn decode(b: &[u8]) -> Result<(), Arc<str>> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    Ok(())
}
