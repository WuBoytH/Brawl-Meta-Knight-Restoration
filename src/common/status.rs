mod fly;
mod glide;
mod damage;

pub fn install() {
    fly::install();
    glide::install();
    damage::install();
}