
#[derive(Debug, Clone)]
pub struct Packet {
    version: u32,
    content: PacketContent,
}

impl Packet {
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
                    Operator::Sum => packets.iter().map(|p| p.evaluate()).sum(),
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
    fn from_type_id(type_id: u32) -> Self {
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

pub struct Transmission {
    bytes: Vec<u8>,
    bit_position: u32,
}

impl Transmission {
    pub fn new(chars: &[char]) -> Self {
        Self {
            bit_position: 0,
            bytes: chars.iter()
                .map(|c| c.to_digit(16).unwrap() as u8)
                .collect()
        }
    }

    pub fn read(&mut self) -> Packet {
        let version = self.get_next(3);
        let type_id = self.get_next(3);

        if type_id == 4 {
            // Literal packet
            let mut literal = 0;
            while self.get_next(1) == 1 {
                literal <<= 4;
                literal |= self.get_next(4) as u64;
            }
            literal <<= 4;
            literal |= self.get_next(4) as u64;

            Packet {
                version,
                content: PacketContent::Literal(literal),
            }
        } else {
            // Operator packet
            let length_type_id = self.get_next(1);
            if length_type_id == 0 {
                // 15 bit length
                let total_length = self.get_next(15);
                let next_pos = self.bit_position + total_length;
                let mut packets = Vec::new();
                while self.bit_position < next_pos {
                    packets.push(self.read());
                }

                Packet {
                    version,
                    content: PacketContent::Operator{ packets, operation: Operator::from_type_id(type_id) },
                }
            } else {
                // 11 bit length
                let mut amount = self.get_next(11);
                let mut packets = Vec::new();
                while amount > 0 {
                    packets.push(self.read());
                    amount -= 1;
                }

                Packet {
                    version,
                    content: PacketContent::Operator{ packets, operation: Operator::from_type_id(type_id) },
                }
            }
        }
    }

    fn get_next(&mut self, n: u32) -> u32 {
        let mut result = 0;
        for i in 0..n {
            let position = self.bit_position + i;

            result <<= 1;
            result |= (self.bytes[position as usize / 4] as u32 >> (3 - (position % 4))) & 1;
        }
        self.bit_position += n;

        result
    }
}




fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .chars()
        .collect();

    let mut input_transmission = Transmission::new(&input);
    let packet = input_transmission.read();

    println!("Version sum: {}", packet.version_sum());
    println!("Evaluation: {}", packet.evaluate());
}
