mod dash;
mod escape;
mod escape_f;
mod escape_b;
mod escape_air;
mod special_hi;

pub fn install() {
    dash::install();
    escape::install();
    escape_f::install();
    escape_b::install();
    escape_air::install();
    special_hi::install();
}