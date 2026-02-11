use super::{
    database::{DbGroup as Db, DbResult, StreamId},
    event::DirectedId,
    recorder::Cx,
};

pub trait DynamicProtocol {
    fn from_name(name: &str, stream_id: StreamId) -> Self;
}

pub trait HandleData {
    // TODO: use Cow for bytes
    fn on_data(&mut self, id: DirectedId, bytes: &mut [u8], cx: &Cx, db: &Db) -> DbResult<()>;
}

mod accumulator;

pub mod mina_protocol;
pub mod mplex;
pub mod multistream_select;
pub mod mux;
pub mod noise;
pub mod pnet;
pub mod yamux;
