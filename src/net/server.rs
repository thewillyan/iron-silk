use std::io::StdinLock;

use serde_json::{de::IoRead, StreamDeserializer};

use anyhow::{bail, Result};

use super::{
    node::Node,
    protocol::{Data, Msg},
};

struct Maelstrom;

impl Maelstrom {
    fn init<'s>() -> Result<MaelstromServer<'s>> {
        let stdin = std::io::stdin().lock();
        let stdout = std::io::stdout().lock();

        let deserializer = serde_json::Deserializer::from_reader(stdin);
        let mut iter = deserializer.into_iter::<Msg>();

        if let Some(inner) = iter.next() {
            let msg = inner?;
            if let Data::Init { node_id, node_ids } = msg.body.data {
            } else {
                bail!(
                    "Expected the first message to be a Init, got an {:?}.",
                    msg.body.data
                )
            }
        }
        todo!();
    }
}

struct MaelstromServer<'s> {
    node: Node,
    stream: StreamDeserializer<'s, IoRead<StdinLock<'static>>, Msg>,
}

impl<'s> MaelstromServer<'s> {
    pub fn serve() {
        let stdin = std::io::stdin().lock();
        let stdout = std::io::stdout().lock();
        unimplemented!()
    }
}
