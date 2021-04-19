use alloc::collections::BTreeMap;

use lazy_static::lazy_static;

use super::{Node, Error, InterfaceId, NodeKind, NodeKindId};
use super::read_write::{IReadable, IWritable, IID_READABLE, IID_WRITABLE};

lazy_static! {
    pub static ref REGULAR_FILE: NodeKind = {
        let mut nk = NodeKind {
            id: NodeKindId::allocate(),
            interfaces: BTreeMap::new()
        };
        nk.interfaces.insert(*IID_READABLE, &REGULAR_FILE_READABLE);
        nk
    };
}

const REGULAR_FILE_READABLE: IReadable = IReadable {
    read: i_readable_read
};

fn i_readable_read(node: &Node) -> Error {
    Error::NotImplemented
}

const REGULAR_FILE_WRITABLE: IWritable = IWritable {
    write: i_writable_write
};

fn i_writable_write(node: &Node) -> Error {
    Error::NotImplemented
}
