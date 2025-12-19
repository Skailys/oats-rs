const DATACENTER_ID_BITS: u8 = 4; // max 7 bits
const NODE_ID_BITS: u8 = 8 - DATACENTER_ID_BITS;

const DATACENTER_ID_MAX: u8 = (1u8 << DATACENTER_ID_BITS) - 1;
const NODE_ID_MAX: u8 = (1u8 << NODE_ID_BITS) - 1;

const NODE_ID_MASK: u8 = (1u8 << NODE_ID_BITS) - 1;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeIdError {
    DatacenterIdTooLong,
    NodeIdTooLong,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeID {
    datacenter_id: u8,
    node_id: u8,
}

pub const NIL: NodeID = NodeID::new(0, 0);
pub const MAX: NodeID = NodeID::new(u8::MAX, u8::MAX);
pub const MAX_SHORT: NodeID = NodeID::new(DATACENTER_ID_MAX, NODE_ID_MAX);

impl NodeID {
    pub const fn new(datacenter_id: u8, node_id: u8) -> Self {
        return NodeID { datacenter_id, node_id };
    }

    pub const fn from_shorted_id(shorted_id: u8) -> Self {
        let datacenter_id = shorted_id >> NODE_ID_BITS;
        let node_id = shorted_id & NODE_ID_MASK;

        return NodeID { datacenter_id, node_id };
    }

    pub const fn from_id(id: u16) -> Self {
        let datacenter_id = (id >> 8) as u8;
        let node_id = (id & 0xFF) as u8;

        return NodeID { datacenter_id, node_id };
    }

    pub const fn datacenter_id(&self) -> u8 {
        self.datacenter_id
    }

    pub const fn node_id(&self) -> u8 {
        self.node_id
    }

    pub const fn short(&self) -> bool {
        return self.datacenter_id <= DATACENTER_ID_MAX && self.node_id <= NODE_ID_MAX;
    }

    pub fn shorted_id(&self) -> Result<u8, NodeIdError> {
        if self.datacenter_id > DATACENTER_ID_MAX {
            return Err(NodeIdError::DatacenterIdTooLong);
        }

        if self.node_id > NODE_ID_MAX {
            return Err(NodeIdError::NodeIdTooLong);
        }

        return Ok(
            (self.datacenter_id << NODE_ID_BITS) | self.node_id
        );
    }

    pub fn id(&self) -> u16 {
        (self.datacenter_id as u16) << 8 | self.node_id as u16
    }

}

// TryInto and From for u8 (shorted IDs)
impl std::convert::TryInto<u8> for NodeID {
    type Error = NodeIdError;

    fn try_into(self) -> Result<u8, Self::Error> {
        self.shorted_id()
    }
}

impl std::convert::From<u8> for NodeID {
    fn from(value: u8) -> Self {
        NodeID::from_shorted_id(value)
    }
}

// Into and From for u16 (longated IDs)
impl std::convert::Into<u16> for NodeID {
    fn into(self) -> u16 {
        self.id()
    }
}

impl std::convert::From<u16> for NodeID {
    fn from(value: u16) -> NodeID {
        NodeID::from_id(value)
    }
}



