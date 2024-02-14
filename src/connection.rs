
#[derive(Clone, Copy)]
pub struct Connection(pub NodeRef, pub NodeRef);

#[derive(Clone, Copy)]
pub struct NodeRef(u8);

