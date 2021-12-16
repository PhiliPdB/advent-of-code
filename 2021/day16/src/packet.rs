
#[derive(Debug, Clone)]
pub struct Packet {
    version: u32,
    content: PacketContent,
}

impl Packet {
    pub fn new(version: u32, content: PacketContent) -> Self {
        Self { version, content }
    }

    pub fn version_sum(&self) -> u32 {
        self.version + self.content.version_sum()
    }

    pub fn evaluate(&self) -> u64 {
        self.content.evaluate()
    }
}

#[derive(Debug, Clone)]
pub enum PacketContent {
    Literal(u64),
    Operator{ operation: Operator, packets: Vec<Packet> },
}

impl PacketContent {
    pub fn version_sum(&self) -> u32 {
        match self {
            PacketContent::Literal(_) => 0,
            PacketContent::Operator { packets, .. } => packets.iter().map(|p| p.version_sum()).sum(),
        }
    }

    pub fn evaluate(&self) -> u64 {
        match self {
            PacketContent::Literal(n) => *n,
            PacketContent::Operator { operation, packets } => {
                match operation {
                    Operator::Sum     => packets.iter().map(|p| p.evaluate()).sum(),
                    Operator::Product => packets.iter().map(|p| p.evaluate()).product(),
                    Operator::Minimum => packets.iter().map(|p| p.evaluate()).min().unwrap(),
                    Operator::Maximum => packets.iter().map(|p| p.evaluate()).max().unwrap(),
                    Operator::GreaterThan => {
                        if packets[0].evaluate() > packets[1].evaluate() {
                            1
                        } else {
                            0
                        }
                    },
                    Operator::LessThan => {
                        if packets[0].evaluate() < packets[1].evaluate() {
                            1
                        } else {
                            0
                        }
                    },
                    Operator::EqualTo => {
                        if packets[0].evaluate() == packets[1].evaluate() {
                            1
                        } else {
                            0
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    pub fn from_type_id(type_id: u32) -> Self {
        match type_id {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Minimum,
            3 => Operator::Maximum,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::EqualTo,
            _ => panic!("Invalid type id"),
        }
    }
}
