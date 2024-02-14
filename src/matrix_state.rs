use std::collections::HashMap;
use strum::EnumIter;

/// Represents one of the 12 crosspoint chips
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
#[repr(u8)]
pub enum Chip {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
}

impl From<char> for Chip {
    fn from(value: char) -> Self {
        use Chip::*;
        match value {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            'E' => E,
            'F' => F,
            'G' => G,
            'H' => H,
            'I' => I,
            'J' => J,
            'K' => K,
            'L' => L,
            _ => panic!("Unknown chip: {:?}", value),
        }
    }
}

/// Captures both the physical connections and current status of a chip
#[derive(Debug)]
pub struct ChipStatus {
    pub chip: Chip,
    pub x_status: [i8; 16],
    pub y_status: [i8; 8],
    pub x_map: [Link; 16],
    pub y_map: [Link; 8],
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum Axis {
    X,
    Y,
}

/// A Link goes either to a [`Node`] or to a specific Xn or Xy port on a [`Chip`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Link {
    Chip(Chip, Axis, usize),
    Node(Node),
}

/// Represents a Node on the board. Nodes are the entities that a user can form connections between.
///
/// "Top" and "Bottom" refer to the two halves of the breadboard.
///
/// The numeric values of the variants correspond to the node IDs used by the jumperless firmware.
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq, EnumIter, PartialOrd, Ord)]
#[repr(u8)]
pub enum Node {
    Top1 = 1,
    Top2 = 2,
    Top3 = 3,
    Top4 = 4,
    Top5 = 5,
    Top6 = 6,
    Top7 = 7,
    Top8 = 8,
    Top9 = 9,
    Top10 = 10,
    Top11 = 11,
    Top12 = 12,
    Top13 = 13,
    Top14 = 14,
    Top15 = 15,
    Top16 = 16,
    Top17 = 17,
    Top18 = 18,
    Top19 = 19,
    Top20 = 20,
    Top21 = 21,
    Top22 = 22,
    Top23 = 23,
    Top24 = 24,
    Top25 = 25,
    Top26 = 26,
    Top27 = 27,
    Top28 = 28,
    Top29 = 29,
    Top30 = 30,
    Bottom1 = 31,
    Bottom2 = 32,
    Bottom3 = 33,
    Bottom4 = 34,
    Bottom5 = 35,
    Bottom6 = 36,
    Bottom7 = 37,
    Bottom8 = 38,
    Bottom9 = 39,
    Bottom10 = 40,
    Bottom11 = 41,
    Bottom12 = 42,
    Bottom13 = 43,
    Bottom14 = 44,
    Bottom15 = 45,
    Bottom16 = 46,
    Bottom17 = 47,
    Bottom18 = 48,
    Bottom19 = 49,
    Bottom20 = 50,
    Bottom21 = 51,
    Bottom22 = 52,
    Bottom23 = 53,
    Bottom24 = 54,
    Bottom25 = 55,
    Bottom26 = 56,
    Bottom27 = 57,
    Bottom28 = 58,
    Bottom29 = 59,
    Bottom30 = 60,

    NanoD0 = 70,
    NanoD1 = 71,
    NanoD2 = 72,
    NanoD3 = 73,
    NanoD4 = 74,
    NanoD5 = 75,
    NanoD6 = 76,
    NanoD7 = 77,
    NanoD8 = 78,
    NanoD9 = 79,
    NanoD10 = 80,
    NanoD11 = 81,
    NanoD12 = 82,
    NanoD13 = 83,
    NanoReset = 84,
    NanoAref = 85,
    NanoA0 = 86,
    NanoA1 = 87,
    NanoA2 = 88,
    NanoA3 = 89,
    NanoA4 = 90,
    NanoA5 = 91,
    NanoA6 = 92,
    NanoA7 = 93,

    Gnd = 100,
    Supply3V3 = 103,
    Supply5V = 105,
    Dac05V = 106,
    Dac18V = 107,
    CurrentSensePlus = 108,
    CurrentSenseMinus = 109,
    Adc05V = 110,
    Adc15V = 111,
    Adc25V = 112,
    Adc38V = 113,
    RpGpio0 = 114,
    RpUartTx = 116,
    RpUartRx = 117,
}

impl From<jlctl::types::Node> for Node {
    fn from(value: jlctl::types::Node) -> Self {
        use Node::*;
        match value {
            jlctl::types::Node::GND => Gnd,
            jlctl::types::Node::SUPPLY_5V => Supply5V,
            jlctl::types::Node::SUPPLY_3V3 => Supply3V3,
            jlctl::types::Node::DAC0 => Dac05V,
            jlctl::types::Node::DAC1 => Dac18V,
            jlctl::types::Node::ISENSE_MINUS => CurrentSenseMinus,
            jlctl::types::Node::ISENSE_PLUS => CurrentSensePlus,
            jlctl::types::Node::ADC0 => Adc05V,
            jlctl::types::Node::ADC1 => Adc15V,
            jlctl::types::Node::ADC2 => Adc25V,
            jlctl::types::Node::ADC3 => Adc38V,
            jlctl::types::Node::NANO_D0 => NanoD0,
            jlctl::types::Node::NANO_D1 => NanoD1,
            jlctl::types::Node::NANO_D2 => NanoD2,
            jlctl::types::Node::NANO_D3 => NanoD3,
            jlctl::types::Node::NANO_D4 => NanoD4,
            jlctl::types::Node::NANO_D5 => NanoD5,
            jlctl::types::Node::NANO_D6 => NanoD6,
            jlctl::types::Node::NANO_D7 => NanoD7,
            jlctl::types::Node::NANO_D8 => NanoD8,
            jlctl::types::Node::NANO_D9 => NanoD9,
            jlctl::types::Node::NANO_D10 => NanoD10,
            jlctl::types::Node::NANO_D11 => NanoD11,
            jlctl::types::Node::NANO_D12 => NanoD12,
            jlctl::types::Node::NANO_D13 => NanoD13,
            jlctl::types::Node::NANO_A0 => NanoA0,
            jlctl::types::Node::NANO_A1 => NanoA1,
            jlctl::types::Node::NANO_A2 => NanoA2,
            jlctl::types::Node::NANO_A3 => NanoA3,
            jlctl::types::Node::NANO_A4 => NanoA4,
            jlctl::types::Node::NANO_A5 => NanoA5,
            jlctl::types::Node::NANO_A6 => NanoA6,
            jlctl::types::Node::NANO_A7 => NanoA7,
            jlctl::types::Node::NANO_RESET => NanoReset,
            jlctl::types::Node::NANO_AREF => NanoAref,
            jlctl::types::Node::RP_GPIO_0 => RpGpio0,
            jlctl::types::Node::RP_UART_Rx => RpUartRx,
            jlctl::types::Node::RP_UART_Tx => RpUartTx,
            jlctl::types::Node::Column(n) => {
                if n >= 1 && n <= 60 {
                    // Safety: values 1..=60 are all valid node values, pointing to the respective breadboard nodes.
                    unsafe { std::mem::transmute(n) }
                } else {
                    panic!("Invalid breadboard node: {:?}", n)
                }
            }
        }
    }
}

/// Represents a connection between a set of ports on the same chip.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Connection {
    pub net: i8,
    pub ports: Vec<(Axis, usize)>,
}

impl ChipStatus {
    fn new(chip: Chip, x_map: [Link; 16], y_map: [Link; 8]) -> Self {
        Self {
            chip,
            x_status: [-1; 16],
            y_status: [-1; 8],
            x_map,
            y_map,
        }
    }

    /// Returns a list of connections currently made by this chip, based on the `x_status`  / `y_status` information.
    pub fn connections(&self) -> Vec<Connection> {
        let mut indices: HashMap<i8, Vec<(Axis, usize)>> = HashMap::new();

        for (i, net) in self.x_status.into_iter().enumerate() {
            if net != -1 {
                let ports = indices.entry(net).or_default();
                ports.push((Axis::X, i));
            }
        }

        for (i, net) in self.y_status.into_iter().enumerate() {
            if net != -1 {
                let ports = indices.entry(net).or_default();
                ports.push((Axis::Y, i));
            }
        }

        return indices
            .into_iter()
            .map(|(net, ports)| Connection { net, ports })
            .collect();
    }

    /// Look up link at the given port (Xn/Yn address).
    ///
    /// `index` must be in the `0..16` range for the `X` axis, and in the `0..8` range for the `Y` axis.
    ///
    /// Panics if the index is out of bounds.
    pub fn lookup_link(&self, axis: Axis, index: usize) -> Link {
        match axis {
            Axis::X if index < 16 => self.x_map[index],
            Axis::Y if index < 8 => self.y_map[index],
            _ => panic!("Port out of bounds: {:?}{}", axis, index),
        }
    }
}

#[derive(Debug)]
pub struct State {
    /// ChipStatus for each of the 12 chips.
    ///
    /// These are always in order, such that `chips[i].chip as u8 == i`.
    pub chips: [ChipStatus; 12],
}

impl State {
    /// Look up link at the given port (chip and Xn/Yn address).
    ///
    /// `index` must be in the `0..16` range for the `X` axis, and in the `0..8` range for the `Y` axis.
    ///
    /// Panics if the index is out of bounds.
    pub fn lookup_link(&self, chip: Chip, axis: Axis, index: usize) -> Link {
        self.chips[chip as u8 as usize].lookup_link(axis, index)
    }

    pub fn set_status(&mut self, chip: Chip, x_status: [i8; 16], y_status: [i8; 8]) {
        let chip = &mut self.chips[chip as u8 as usize];
        chip.x_status = x_status;
        chip.y_status = y_status;
    }

    /// Extract a list of nets that are currently formed by the matrix.
    pub fn extract_nets(&self) -> Vec<(i8, Vec<Node>)> {
        let mut nets: HashMap<i8, Vec<Node>> = HashMap::new();

        for cs in &self.chips {
            for connection in cs.connections() {
                let nodes = connection.ports.iter().filter_map(|port| {
                    match self.lookup_link(cs.chip, port.0, port.1) {
                        Link::Node(node) => Some(node),
                        _ => None,
                    }
                });
                nets.entry(connection.net).or_default().extend(nodes);
            }
        }

        let mut nets: Vec<_> = nets.into_iter().collect();
        nets.sort();
        nets
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Netlist(Vec<(i8, Vec<Node>)>);

impl Default for State {
    fn default() -> Self {
        use Axis::*;
        use Chip::*;
        use Node::*;

        Self {
            chips: [
                ChipStatus::new(
                    A,
                    [
                        Link::Chip(I, Y, 0),
                        Link::Chip(J, Y, 0),
                        Link::Chip(B, X, 0),
                        Link::Chip(B, X, 1),
                        Link::Chip(C, X, 0),
                        Link::Chip(C, X, 1),
                        Link::Chip(D, X, 0),
                        Link::Chip(D, X, 1),
                        Link::Chip(E, X, 0),
                        Link::Chip(K, Y, 0),
                        Link::Chip(F, X, 0),
                        Link::Chip(F, X, 1),
                        Link::Chip(G, X, 0),
                        Link::Chip(G, X, 1),
                        Link::Chip(H, X, 0),
                        Link::Chip(H, X, 1),
                    ],
                    [
                        Link::Chip(L, Y, 0),
                        Link::Node(Top2),
                        Link::Node(Top3),
                        Link::Node(Top4),
                        Link::Node(Top5),
                        Link::Node(Top6),
                        Link::Node(Top7),
                        Link::Node(Top8),
                    ],
                ),
                ChipStatus::new(
                    B,
                    [
                        Link::Chip(A, X, 2),
                        Link::Chip(A, X, 3),
                        Link::Chip(I, Y, 1),
                        Link::Chip(J, Y, 1),
                        Link::Chip(C, X, 2),
                        Link::Chip(C, X, 3),
                        Link::Chip(D, X, 2),
                        Link::Chip(D, X, 3),
                        Link::Chip(E, X, 2),
                        Link::Chip(E, X, 3),
                        Link::Chip(F, X, 2),
                        Link::Chip(K, Y, 1),
                        Link::Chip(G, X, 2),
                        Link::Chip(G, X, 3),
                        Link::Chip(H, X, 2),
                        Link::Chip(H, X, 3),
                    ],
                    [
                        Link::Chip(L, Y, 1),
                        Link::Node(Top9),
                        Link::Node(Top10),
                        Link::Node(Top11),
                        Link::Node(Top12),
                        Link::Node(Top13),
                        Link::Node(Top14),
                        Link::Node(Top15),
                    ],
                ),
                ChipStatus::new(
                    C,
                    [
                        Link::Chip(A, X, 4),
                        Link::Chip(A, X, 5),
                        Link::Chip(B, X, 4),
                        Link::Chip(B, X, 5),
                        Link::Chip(I, Y, 2),
                        Link::Chip(J, Y, 2),
                        Link::Chip(D, X, 4),
                        Link::Chip(D, X, 5),
                        Link::Chip(E, X, 4),
                        Link::Chip(E, X, 5),
                        Link::Chip(F, X, 4),
                        Link::Chip(F, X, 5),
                        Link::Chip(G, X, 4),
                        Link::Chip(K, Y, 2),
                        Link::Chip(H, X, 4),
                        Link::Chip(H, X, 5),
                    ],
                    [
                        Link::Chip(L, Y, 2),
                        Link::Node(Top16),
                        Link::Node(Top17),
                        Link::Node(Top18),
                        Link::Node(Top19),
                        Link::Node(Top20),
                        Link::Node(Top21),
                        Link::Node(Top22),
                    ],
                ),
                ChipStatus::new(
                    D,
                    [
                        Link::Chip(A, X, 6),
                        Link::Chip(A, X, 7),
                        Link::Chip(B, X, 6),
                        Link::Chip(B, X, 7),
                        Link::Chip(C, X, 6),
                        Link::Chip(C, X, 7),
                        Link::Chip(I, Y, 3),
                        Link::Chip(J, Y, 3),
                        Link::Chip(E, X, 6),
                        Link::Chip(E, X, 7),
                        Link::Chip(F, X, 6),
                        Link::Chip(F, X, 7),
                        Link::Chip(G, X, 6),
                        Link::Chip(G, X, 7),
                        Link::Chip(H, X, 6),
                        Link::Chip(K, Y, 3),
                    ],
                    [
                        Link::Chip(L, Y, 3),
                        Link::Node(Top23),
                        Link::Node(Top24),
                        Link::Node(Top25),
                        Link::Node(Top26),
                        Link::Node(Top27),
                        Link::Node(Top28),
                        Link::Node(Top29),
                    ],
                ),
                ChipStatus::new(
                    E,
                    [
                        Link::Chip(A, X, 8),
                        Link::Chip(K, Y, 4),
                        Link::Chip(B, X, 8),
                        Link::Chip(B, X, 9),
                        Link::Chip(C, X, 8),
                        Link::Chip(C, X, 9),
                        Link::Chip(D, X, 8),
                        Link::Chip(D, X, 9),
                        Link::Chip(I, Y, 4),
                        Link::Chip(J, Y, 4),
                        Link::Chip(F, X, 8),
                        Link::Chip(F, X, 9),
                        Link::Chip(G, X, 8),
                        Link::Chip(G, X, 9),
                        Link::Chip(H, X, 8),
                        Link::Chip(H, X, 9),
                    ],
                    [
                        Link::Chip(L, Y, 4),
                        Link::Node(Bottom2),
                        Link::Node(Bottom3),
                        Link::Node(Bottom4),
                        Link::Node(Bottom5),
                        Link::Node(Bottom6),
                        Link::Node(Bottom7),
                        Link::Node(Bottom8),
                    ],
                ),
                ChipStatus::new(
                    F,
                    [
                        Link::Chip(A, X, 10),
                        Link::Chip(A, X, 11),
                        Link::Chip(B, X, 10),
                        Link::Chip(K, Y, 5),
                        Link::Chip(C, X, 10),
                        Link::Chip(C, X, 11),
                        Link::Chip(D, X, 10),
                        Link::Chip(D, X, 11),
                        Link::Chip(E, X, 10),
                        Link::Chip(E, X, 11),
                        Link::Chip(I, Y, 5),
                        Link::Chip(J, Y, 5),
                        Link::Chip(G, X, 10),
                        Link::Chip(G, X, 11),
                        Link::Chip(H, X, 10),
                        Link::Chip(H, X, 11),
                    ],
                    [
                        Link::Chip(L, Y, 5),
                        Link::Node(Bottom9),
                        Link::Node(Bottom10),
                        Link::Node(Bottom11),
                        Link::Node(Bottom12),
                        Link::Node(Bottom13),
                        Link::Node(Bottom14),
                        Link::Node(Bottom15),
                    ],
                ),
                ChipStatus::new(
                    G,
                    [
                        Link::Chip(A, X, 12),
                        Link::Chip(A, X, 13),
                        Link::Chip(B, X, 12),
                        Link::Chip(B, X, 13),
                        Link::Chip(C, X, 12),
                        Link::Chip(K, Y, 6),
                        Link::Chip(D, X, 12),
                        Link::Chip(D, X, 13),
                        Link::Chip(E, X, 12),
                        Link::Chip(E, X, 13),
                        Link::Chip(F, X, 12),
                        Link::Chip(F, X, 13),
                        Link::Chip(I, Y, 6),
                        Link::Chip(J, Y, 6),
                        Link::Chip(H, X, 12),
                        Link::Chip(H, X, 13),
                    ],
                    [
                        Link::Chip(L, Y, 6),
                        Link::Node(Bottom16),
                        Link::Node(Bottom17),
                        Link::Node(Bottom18),
                        Link::Node(Bottom19),
                        Link::Node(Bottom20),
                        Link::Node(Bottom21),
                        Link::Node(Bottom22),
                    ],
                ),
                ChipStatus::new(
                    H,
                    [
                        Link::Chip(A, X, 14),
                        Link::Chip(A, X, 15),
                        Link::Chip(B, X, 14),
                        Link::Chip(B, X, 15),
                        Link::Chip(C, X, 14),
                        Link::Chip(C, X, 15),
                        Link::Chip(D, X, 14),
                        Link::Chip(K, Y, 7),
                        Link::Chip(E, X, 14),
                        Link::Chip(E, X, 15),
                        Link::Chip(F, X, 14),
                        Link::Chip(F, X, 15),
                        Link::Chip(G, X, 14),
                        Link::Chip(G, X, 15),
                        Link::Chip(I, Y, 7),
                        Link::Chip(J, Y, 7),
                    ],
                    [
                        Link::Chip(L, Y, 7),
                        Link::Node(Bottom23),
                        Link::Node(Bottom24),
                        Link::Node(Bottom25),
                        Link::Node(Bottom26),
                        Link::Node(Bottom27),
                        Link::Node(Bottom28),
                        Link::Node(Bottom29),
                    ],
                ),
                ChipStatus::new(
                    I,
                    [
                        Link::Node(NanoA0),
                        Link::Node(NanoD1),
                        Link::Node(NanoA2),
                        Link::Node(NanoD3),
                        Link::Node(NanoA4),
                        Link::Node(NanoD5),
                        Link::Node(NanoA6),
                        Link::Node(NanoD7),
                        Link::Node(NanoD11),
                        Link::Node(NanoD9),
                        Link::Node(NanoD13),
                        Link::Node(NanoReset),
                        Link::Node(Dac05V),
                        Link::Node(Adc05V),
                        Link::Node(Supply3V3),
                        Link::Node(Gnd),
                    ],
                    [
                        Link::Chip(A, X, 0),
                        Link::Chip(B, X, 2),
                        Link::Chip(C, X, 4),
                        Link::Chip(D, X, 6),
                        Link::Chip(E, X, 8),
                        Link::Chip(F, X, 10),
                        Link::Chip(G, X, 12),
                        Link::Chip(H, X, 14),
                    ],
                ),
                ChipStatus::new(
                    J,
                    [
                        Link::Node(NanoD0),
                        Link::Node(NanoA1),
                        Link::Node(NanoD2),
                        Link::Node(NanoA3),
                        Link::Node(NanoD4),
                        Link::Node(NanoA5),
                        Link::Node(NanoD6),
                        Link::Node(NanoA7),
                        Link::Node(NanoD8),
                        Link::Node(NanoD10),
                        Link::Node(NanoD12),
                        Link::Node(NanoAref),
                        Link::Node(Dac18V),
                        Link::Node(Adc15V),
                        Link::Node(Supply5V),
                        Link::Node(Gnd),
                    ],
                    [
                        Link::Chip(A, X, 1),
                        Link::Chip(B, X, 3),
                        Link::Chip(C, X, 5),
                        Link::Chip(D, X, 7),
                        Link::Chip(E, X, 9),
                        Link::Chip(F, X, 11),
                        Link::Chip(G, X, 13),
                        Link::Chip(H, X, 15),
                    ],
                ),
                ChipStatus::new(
                    K,
                    [
                        Link::Node(NanoA0),
                        Link::Node(NanoA1),
                        Link::Node(NanoA2),
                        Link::Node(NanoA3),
                        Link::Node(NanoD2),
                        Link::Node(NanoD3),
                        Link::Node(NanoD4),
                        Link::Node(NanoD5),
                        Link::Node(NanoD6),
                        Link::Node(NanoD7),
                        Link::Node(NanoD8),
                        Link::Node(NanoD9),
                        Link::Node(NanoD10),
                        Link::Node(NanoD11),
                        Link::Node(NanoD12),
                        Link::Node(Adc25V),
                    ],
                    [
                        Link::Chip(A, X, 9),
                        Link::Chip(B, X, 11),
                        Link::Chip(C, X, 13),
                        Link::Chip(D, X, 15),
                        Link::Chip(E, X, 1),
                        Link::Chip(F, X, 3),
                        Link::Chip(G, X, 5),
                        Link::Chip(H, X, 7),
                    ],
                ),
                ChipStatus::new(
                    L,
                    [
                        Link::Node(CurrentSenseMinus),
                        Link::Node(CurrentSensePlus),
                        Link::Node(Adc05V),
                        Link::Node(Adc15V),
                        Link::Node(Adc25V),
                        Link::Node(Adc38V),
                        Link::Node(Dac18V),
                        Link::Node(Dac05V),
                        Link::Node(Top1),
                        Link::Node(Top30),
                        Link::Node(Bottom1),
                        Link::Node(Bottom30),
                        Link::Node(RpUartTx),
                        Link::Node(RpUartRx),
                        Link::Node(Supply5V),
                        Link::Node(RpGpio0),
                    ],
                    [
                        Link::Chip(A, Y, 0),
                        Link::Chip(B, Y, 0),
                        Link::Chip(C, Y, 0),
                        Link::Chip(D, Y, 0),
                        Link::Chip(E, Y, 0),
                        Link::Chip(F, Y, 0),
                        Link::Chip(G, Y, 0),
                        Link::Chip(H, Y, 0),
                    ],
                ),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_xy_map_sanity() {
        let state = State::default();

        // keep track of how many links each node has
        let mut node_map: HashMap<Node, usize> = HashMap::new();

        // For every link of every chip, either:
        // - increment the node-link counter, if the link goes to a node
        // - look up the opposite link, if the link goes to a chip, and check that that link points back to the link currently checked.
        for cs in &state.chips {
            for (i, link) in cs.x_map.into_iter().enumerate() {
                print!("Check {:?} X{}: ", cs.chip, i);
                match link {
                    Link::Node(node) => *node_map.entry(node).or_default() += 1,
                    Link::Chip(chip, axis, index) => {
                        let opposite = state.lookup_link(chip, axis, index);
                        assert_eq!(
                            opposite,
                            Link::Chip(cs.chip, Axis::X, i),
                            "Left: link at {:?}, Right: currently checked link",
                            (chip, axis, index)
                        );
                    }
                }
                println!("ok");
            }

            for (i, link) in cs.y_map.into_iter().enumerate() {
                print!("Check {:?} Y{}: ", cs.chip, i);
                match link {
                    Link::Node(node) => *node_map.entry(node).or_default() += 1,
                    Link::Chip(chip, axis, index) => {
                        let opposite = state.lookup_link(chip, axis, index);
                        assert_eq!(
                            opposite,
                            Link::Chip(cs.chip, Axis::Y, i),
                            "Left: link at {:?}, Right: currently checked link",
                            (chip, axis, index)
                        );
                    }
                }
                println!("ok");
            }
        }

        // check that all nodes show up in at least one link
        for node in Node::iter() {
            let count = node_map.entry(node).or_default();
            println!("{:?}: {}", node, count);
            assert!(
                *count > 0,
                "node {:?} should be part of at least one link",
                node
            );
        }
    }

    #[test]
    fn test_connections() {
        let mut state = State::default();
        let chip = &mut state.chips[0];

        assert_eq!(chip.connections(), vec![]);

        chip.x_status[3] = 17;
        chip.y_status[7] = 17;

        assert_eq!(
            chip.connections(),
            vec![Connection {
                net: 17,
                ports: vec![(Axis::X, 3), (Axis::Y, 7)],
            },]
        );

        chip.x_status[4] = 19;
        chip.x_status[5] = 19;
        chip.y_status[3] = 19;

        let mut connections = chip.connections();
        connections.sort();

        assert_eq!(
            connections,
            vec![
                Connection {
                    net: 17,
                    ports: vec![(Axis::X, 3), (Axis::Y, 7)],
                },
                Connection {
                    net: 19,
                    ports: vec![(Axis::X, 4), (Axis::X, 5), (Axis::Y, 3)],
                },
            ]
        );
    }
}
