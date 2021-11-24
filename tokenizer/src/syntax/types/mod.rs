pub mod array_type;
pub mod brace_reference_type;
pub mod char_type;
pub mod float_type;
pub mod function_call_type;
pub mod integer_type;
pub mod negative_type;
pub mod operator_type;
pub mod reference_type;
pub mod string_type;
pub mod variable_type;

pub trait Converter<F, T> {
    fn to_definite(self) -> T;
    fn from_definite(self, from: T) -> F;
}
