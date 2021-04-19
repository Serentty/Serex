use lazy_static::lazy_static;

use super::{Node, Error, InterfaceId, InterfaceTable};

lazy_static! {
    pub static ref IID_READABLE: InterfaceId = InterfaceId::allocate();
    pub static ref IID_WRITABLE: InterfaceId = InterfaceId::allocate();
}

pub struct IReadable {
    pub read: fn(&Node) -> Error
}

impl InterfaceTable for IReadable {
    fn id() -> InterfaceId {
	*IID_READABLE
    }
}

pub struct IWritable {
    pub write: fn(&Node) -> Error
}

impl InterfaceTable for IWritable {
    fn id() -> InterfaceId {
	*IID_WRITABLE
    }
}
