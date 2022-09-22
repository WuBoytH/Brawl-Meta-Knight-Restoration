mod glide_checks;
mod glide;

pub fn install() {
    glide_checks::install();
    glide::install();
}