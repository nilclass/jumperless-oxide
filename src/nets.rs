use crate::connection::{Connection, NodeRef};

pub struct Net {
}

pub struct Nets<const N: usize = 128>([Net; N]);

pub enum Error {
    DoNotIntersect,
}

enum Action {
    None,
    // combine two nets into one
    MergeNets(usize, usize),
    // adds a node to an existing net
    AddToNet(usize, NodeRef),
    // creates a new net
    NewNet,
}

impl Nets {
    fn make_connection(&mut self, connection: Connection) -> Result<(), Error> {
        let action = self.connection_action(connection)?;

        match action {
            Action::MergeNets(i, j) => {
                while let Some(node) = self.0[j].pop_node() {
                    self.0[i].add_node(node);
                }
            }
            Action::AddToNet(i, node) => {
                self.0[i].add_node(node);
            }
            Action::NewNet => {
                for net in &mut self.0 {
                    if net.is_empty() {
                        net.add_node(connection.0);
                        net.add_node(connection.1);
                        break;
                    }
                }
            }
            Action::None => {}
        }

        Ok(())
    }

    fn connection_action(&self, connection: Connection) -> Result<Action, Error> {
        // 1. Find existing nets of nodes
        let mut net_a = None;
        let mut net_b = None;
        for (i, net) in self.0.iter().enumerate() {
            if net.has_node(connection.0) {
                net_a = Some(i);
            }
            if net.has_node(connection.1) {
                net_b = Some(i);
            }
        }

        // 2. Determine what to do
        let mut action = Action::None;
        match (net_a, net_b) {
            // nodes are connected already (part of the same net)
            (Some(i), Some(j)) if i == j => Ok(Action::None),
            // nodes are part of different nets
            (Some(i), Some(j)) => {
                if self.0[i].can_intersect(&self.0[j]) {
                    Ok(Action::MergeNets(i, j))
                } else {
                    Err(Error::DoNotIntersect)
                }
            }
            // one of the nodes is already in a net, the other is not
            (Some(i), None) | (None, Some(i)) => Ok(Action::AddToNet(i, connection.1)),
            // none of the nodes are in a net
            (None, None) => Ok(Action::NewNet),
        }
    }
}

impl Net {
    pub fn can_intersect(&self, other: &Net) -> bool {
        // FIXME: stub!
        true
    }

    pub fn is_empty(&self) -> bool {
        // FIXME: stub!
        true
    }

    pub fn has_node(&self, node: NodeRef) -> bool {
        false
    }

    pub fn add_node(&mut self, node: NodeRef) {
        todo!()
    }

    pub fn pop_node(&mut self) -> Option<NodeRef> {
        todo!()
    }
}
