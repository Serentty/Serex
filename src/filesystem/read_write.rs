use lazy_static::lazy_static;

use super::{Node, Error, InterfaceId, InterfaceTable};

lazy_static! {
    pub static ref IID_READABLE: InterfaceId = InterfaceId::allocate();
    pub static ref IID_WRITABLE: InterfaceId = InterfaceId::allocate();
}

pub struct Readable {
    pub read: fn(&Node) -> Error
}

impl InterfaceTable for Readable {}

pub struct Writable {
    pub write: fn(&Node) -> Error
}

impl InterfaceTable for Writable {}