use std::cmp::Ordering;
use rand::Rng;

pub trait Property {
    fn fulfilled(&self, by: u8) -> bool;
    fn debug_print(&self);
    fn generate() -> Self;
}

// pub fn create_set(length: u8) -> Vec<Prop> {
//     Vec::<Prop>::new()
// }

/*
Goals:
 Variety of Properties that are easy to generate,
 create new properties, and check fulfilled status.
 All properties should be able to be stored in one object
 with unknown length and data types, which are only determined
 while the object is constructed, by the generate method.

*/

pub struct Problem {
    properties: Vec<GeneralProperty>
    // It is a collection of a bunch of things that inherit Property trait
    // I want to be able to easily loop through it and call functions defined by Property
}

impl Problem {
    fn generate(length: u8) -> Self {
        let mut properties: Vec<GeneralProperty> = Vec::new();
        for i in 0..length {
            properties.push(GeneralProperty::generate(i % 4));
        }
        Problem {
            properties
        }
    }

    fn check(&self, num: u8) -> u8 {
        let mut i = 0;
        let mut sum: u8 = 0;
        while i < self.properties.len() {
            if self.properties[i].fulfilled(num) {
                sum |= 1 << i;
            }

            i += 1;
        }
        sum
    }
}

pub enum GeneralProperty {
    HasBase(HasBase),
    SumDigits(SumDigits),
    NumDigits(NumDigits),
    HasFactor(HasFactor),
    NumFactor(NumFactor),
    MaxGCDWith(MaxGCDWith),
    Prime(Prime),
    Sequence(Sequence),
}

impl GeneralProperty {
    fn fulfilled(&self, by: u8) -> bool {
        match self {
            GeneralProperty::HasBase(prop) => prop.fulfilled(by),
            GeneralProperty::SumDigits(prop) => prop.fulfilled(by),
            GeneralProperty::NumDigits(prop) => prop.fulfilled(by),
            GeneralProperty::HasFactor(prop) => prop.fulfilled(by),
            GeneralProperty::NumFactor(prop) => prop.fulfilled(by),
            GeneralProperty::MaxGCDWith(prop) => prop.fulfilled(by),
            GeneralProperty::Prime(prop) => prop.fulfilled(by),
            GeneralProperty::Sequence(prop) => prop.fulfilled(by),
        }
    }

    fn debug_print(&self) {
        match self {
            GeneralProperty::HasBase(prop) => prop.debug_print(),
            GeneralProperty::SumDigits(prop) => prop.debug_print(),
            GeneralProperty::NumDigits(prop) => prop.debug_print(),
            GeneralProperty::HasFactor(prop) => prop.debug_print(),
            GeneralProperty::NumFactor(prop) => prop.debug_print(),
            GeneralProperty::MaxGCDWith(prop) => prop.debug_print(),
            GeneralProperty::Prime(prop) => prop.debug_print(),
            GeneralProperty::Sequence(prop) => prop.debug_print(),
        }
    }

    fn generate(family: u8) -> Self {
        GeneralProperty::HasBase(HasBase::generate())
    }
}

// pub enum Representation {
//     BaseRep(HasBase),
//     SumDigits(SumDigits),
//     NumDigits(NumDigits),
// }

// pub enum Factor {
//     HasFactor(HasFactor),
//     NumFactor(NumFactor),
//     MaxGCDWith(MaxGCDWith),
//     Prime(Prime),
// }

pub enum Sequence {
    Triangular(bool),
    Fibbonacci(bool),
    TwoPowers(bool),
    SquareNums(bool),
}

impl Property for Sequence {
    fn fulfilled(&self, by:u8) -> bool {
        match self {
            Sequence::Triangular(included) => !(included ^ check_in_triangular(by)),
            Sequence::Fibbonacci(included) => !(included ^ check_in_fibbonacci(by)),
            Sequence::TwoPowers(included)  => !(included ^ check_in_two_powers(by)),
            Sequence::SquareNums(included) => !(included ^ check_in_square_num(by)),
        }
    }

    fn debug_print(&self) {
        println!("Input is in the {} sequence", match self {
            Sequence::Triangular(_) => "triangular",
            Sequence::Fibbonacci(_) => "fibbonacci",
            Sequence::TwoPowers(_)  => "powers of two",
            Sequence::SquareNums(_) => "square numbers",
        });
    }

    fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let rnum = rng.gen_range(0..4);
        let rbool = rng.gen_bool(0.5);
        match rnum {
            0 => Sequence::Triangular(rbool),
            1 => Sequence::Fibbonacci(rbool),
            2 => Sequence::TwoPowers(rbool),
            3 => Sequence::SquareNums(rbool),
            _ => unreachable!()
        }
    }
}

pub struct HasBase {
    character: char, // Which Character to contain
    amount: u8, // How many of the character (0..=3)
    base: u8,
}

// Needs Implementation
impl Property for HasBase {
    fn fulfilled(&self, by: u8) -> bool {
        false
    }

    fn debug_print(&self) {
        println!("Input contains character \'{}\' {} times in base {}", self.character, self.amount, self.base);
    }

    fn generate() -> Self {
        HasBase {
            character: ' ',
            amount: 0,
            base: 10
        }
    }
}

pub struct HasFactor {
    factor: u8,
    contains: bool,
}

impl Property for HasFactor {
    fn fulfilled(&self, by: u8) -> bool {
        false
    }

    fn debug_print(&self) {
        // println!("Input contains character \'{}\' {} times in base {}", self.character, self.amount, self.base);
    }

    fn generate() -> Self {
        HasFactor {
            factor: 0,
            contains: false,
        }
    }
}

pub struct NumFactor {
    num: u8,
    comparison: Ordering,
}

impl Property for NumFactor {
    fn fulfilled(&self, by: u8) -> bool {
        false
    }

    fn debug_print(&self) {
        // println!("Input contains character \'{}\' {} times in base {}", self.character, self.amount, self.base);
    }

    fn generate() -> Self {
        NumFactor {
            num: 0,
            comparison: Ordering::Equal,
        }
    }
}

pub struct MaxGCDWith {
    with: u8,
    border: u8,
    comparison: Ordering,
}

impl Property for MaxGCDWith {
    fn fulfilled(&self, by: u8) -> bool {
        false
    }

    fn debug_print(&self) {
        // println!("Input contains character \'{}\' {} times in base {}", self.character, self.amount, self.base);
    }

    fn generate() -> Self {
        MaxGCDWith {
            with: 32,
            border: 2,
            comparison: Ordering::Greater
        }
    }
}

pub struct Prime {
    is_prime: bool,
}

impl Property for Prime {
    fn fulfilled(&self, by: u8) -> bool {
        false
    }

    fn debug_print(&self) {
       // println!("Input contains character \'{}\' {} times in base {}", self.character, self.amount, self.base);
    }

    fn generate() -> Self {
        Prime {
            is_prime: false,
        }
    }
}

pub struct NumDigits {
    amount: u8,
    comparison: Ordering,
}

// Needs Implementation
impl Property for NumDigits {
    fn fulfilled(&self, by: u8) -> bool {
        false
    }

    fn debug_print(&self) {
        println!("# Digits {} {}", match self.comparison {
                                        Ordering::Greater => ">",
                                        Ordering::Equal => "=",
                                        Ordering::Less => "<",
                                    }, self.amount);
    }

    fn generate() -> Self {
        let amount = rand::thread_rng().gen_range(0..=3);
        let rnum = rand::thread_rng().gen_range(0..2);
        let comparison: Ordering = if amount <= 1 {Ordering::Greater}
                                   else if amount == 3 {Ordering::Less}
                                   else {vec![Ordering::Greater,Ordering::Equal][rnum]};
        NumDigits {
            amount,
            comparison,
        }
    }
}

pub struct SumDigits {
    sum: u8,
    comparison: Ordering,
}

// Needs Implementation
impl Property for SumDigits {
    fn fulfilled(&self, by: u8) -> bool {
        false
    }

    fn debug_print(&self) {
        println!("Digits sum to {} {}", match self.comparison {
            Ordering::Greater => "more than",
            Ordering::Equal => "exactly",
            Ordering::Less => "less than",
        }, self.sum);
    }

    // pick random number, if <6 make it a greater than, if > 16 make it a less than, otherwise choose randomly between the three options
    fn generate() -> Self {
        let sum = rand::thread_rng().gen_range(4..=24);
        let rnum = rand::thread_rng().gen_range(0..3);
        let comparison: Ordering = if sum < 6 {Ordering::Greater}
                                   else if sum > 16 {Ordering::Less}
                                   else {vec![Ordering::Greater,Ordering::Equal,Ordering::Less][rnum]};
        SumDigits {
            sum,
            comparison,
        }
    }
}

// Helper Functions
fn check_in_triangular(by: u8) -> bool {
    for x in 0..=22 {
        if x*x+x == 2*by { return true; }
    }
    false
}

fn check_in_fibbonacci(by: u8) -> bool {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..=14 {
        let c = a + b;
        if c == by {return true;}
        a = b;
        b = c;
    }
    false
}

fn check_in_two_powers(by: u8) -> bool {
    let mut a = 1;
    for _ in 0..7 {
        if a == by {return true;}
        a *= 2;
    }
    false
}

fn check_in_square_num(by: u8) -> bool {
    let mut a = 1;
    while a < 16 {
        if a*a == by {return true;}
        a += 1;
    }
    false
}