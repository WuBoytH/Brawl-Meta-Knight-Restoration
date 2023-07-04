mod dash;
mod escape;
mod escape_f;
mod escape_b;
mod escape_air;
mod attack_dash;
mod special_hi;
mod glide;

pub fn install() {
    dash::install();
    escape::install();
    escape_f::install();
    escape_b::install();
    escape_air::install();
    attack_dash::install();
    special_hi::install();
    glide::install();
}