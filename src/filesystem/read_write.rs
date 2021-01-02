use lazy_static::lazy_static;

use super::{Node, Error, InterfaceId, InterfaceTable};

pub struct Readable {
    pub read: fn(&Node) -> Error
}

impl InterfaceTable for Readable {}

pub struct Writable {
    pub write: fn(&Node) -> Error
}

impl InterfaceTable for Writable {}