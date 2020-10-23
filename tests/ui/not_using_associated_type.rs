#![warn(clippy::not_using_associated_type)]

pub enum State {
    A,
    B,
    C,
    D(u32),
}

// Example: match arms reference State
pub struct TestWithAssociatedState;

pub trait TraitWithOneAssociatedType {
    type Associated;
    fn do_something(&self, associated: &Self::Associated);
}

impl TraitWithOneAssociatedType for TestWithAssociatedState {
    type Associated = State;
    fn do_something(&self, associated: &Self::Associated) {
        match associated {
            // These lines should warn
            State::A => println!("A!"),
            State::B => println!("B!"),
            State::C => println!("C!"),
            State::D(x) => println!("D!"),

            _ => println!("Something else"),
        }
    }
}

// Another example where the same type as the associated type is used elsewhere
pub struct TestWithConcreteAndAssociatedType;

pub trait TraitWithConcreteAndAssociatedType {
    type Associated;
    fn do_something_with_two_values(&self, source: &Self::Associated, target: State) -> u32;
}

impl TraitWithConcreteAndAssociatedType for TestWithConcreteAndAssociatedType {
    type Associated = State;
    fn do_something_with_two_values(&self, source: &Self::Associated, target: State) -> u32 {
        let target_number = match target {
            // These lines should be allowed
            State::A => 2,
            State::B => 4,
            State::C => 6,
            _ => 8,
        };
        let source_number = match source {
            // These lines should warn
            State::A => 1,
            State::B => 3,
            State::C => 5,
            _ => 7,
        };
        target_number + source_number
    }
}

// An example where the Associated type is used in the parameter pattern

pub enum Value {
    Value(u32),
}

pub struct TestWithAssociatedTypeInParameter;

pub trait TraitWithAssociatedTypeInParameter {
    type Associated;
    fn do_something(&self, _: &Self::Associated);
}

impl TraitWithAssociatedTypeInParameter for TestWithAssociatedTypeInParameter {
    type Associated = Value;
    // Should warn on "Value::Value(_state)"
    fn do_something(&self, Value::Value(_state) : &Self::Associated) {
        println!("Hello");
    }
}

fn main() {}
