use alloc::collections::BTreeMap;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;

use lazy_static::lazy_static;
use spin::Mutex;

pub mod read_write;
mod regular_file;

pub type Count = u64;
pub type AMNode<'a> = Arc<Mutex<Node<'a>>>;
pub type MNode<'a> = Mutex<Node<'a>>;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeId(u64);

impl NodeId {
    fn allocate() -> Self {
        let mut next = *NEXT_FREE_NODE_ID.lock();
        let current = next;
        next.0 += 1;
        current
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct InterfaceId(u64);

impl InterfaceId {
    fn allocate() -> Self {
        let mut next = *NEXT_FREE_INTERFACE_ID.lock();
        let current = next;
        next.0 += 1;
        current
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeKindId(u64);

impl NodeKindId {
    fn allocate() -> Self {
        let mut next = *NEXT_FREE_NODE_KIND_ID.lock();
        let current = next;
        next.0 += 1;
        current
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    NotImplemented
}

pub trait InterfaceTable : Sync + mopa::Any {
    fn id() -> InterfaceId where Self: Sized;
}

mopafy!(InterfaceTable, core = core, alloc = alloc);

pub struct NodeKind {
    id: NodeKindId,
    interfaces: BTreeMap<InterfaceId, &'static dyn InterfaceTable>
}

pub struct Node<'a> {
    id: NodeId,
    parent: Option<Weak<MNode<'a>>>,
    kind: &'static NodeKind
}

impl Node<'_> {
    pub fn new_regular_file() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Node {
            id: NodeId::allocate(),
            parent: None,
            kind: &regular_file::REGULAR_FILE
        }))
    }

    pub fn query_dynamic(&self, interface: InterfaceId) -> Option<&'static dyn InterfaceTable> {
        if let Some(table) = self.kind.interfaces.get(&interface) {
            Some(*table)
        } else {
            None
        }
    }

    pub fn query<T: InterfaceTable>(&self) -> Option<&'static T> {
        self.query_dynamic(T::id()).and_then(|table| table.downcast_ref::<T>())
    }
}

lazy_static! {
    static ref NEXT_FREE_NODE_ID: Mutex<NodeId> = Mutex::new(NodeId(1));
}

lazy_static! {
    static ref NEXT_FREE_INTERFACE_ID: Mutex<InterfaceId> = Mutex::new(InterfaceId(0));
}

lazy_static! {
    static ref NEXT_FREE_NODE_KIND_ID: Mutex<NodeKindId> = Mutex::new(NodeKindId(0));
}
