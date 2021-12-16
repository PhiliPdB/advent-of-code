use packet::{Packet, PacketContent, Operator};

pub mod packet;


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

            Packet::new(version, PacketContent::Literal(literal))
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

                Packet::new(
                    version,
                    PacketContent::Operator{ packets, operation: Operator::from_type_id(type_id) }
                )
            } else {
                // 11 bit length
                let mut amount = self.get_next(11);
                let mut packets = Vec::new();
                while amount > 0 {
                    packets.push(self.read());
                    amount -= 1;
                }

                Packet::new(
                    version,
                    PacketContent::Operator{ packets, operation: Operator::from_type_id(type_id) }
                )
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
