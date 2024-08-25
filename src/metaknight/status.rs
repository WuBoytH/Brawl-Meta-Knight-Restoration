mod dash;
mod escape;
mod escape_f;
mod escape_b;
mod escape_air;
mod attack_dash;
mod special_n;
mod special_hi;
mod special_lw;
mod glide;

pub fn install(agent: &mut smashline::Agent) {
    dash::install(agent);
    escape::install(agent);
    escape_f::install(agent);
    escape_b::install(agent);
    escape_air::install(agent);
    attack_dash::install(agent);
    special_n::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    glide::install(agent);
}
