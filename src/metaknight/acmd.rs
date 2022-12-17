mod normals;
mod smashes;
mod aerials;
mod catch;
mod throws;
mod specials;
mod glide;

pub fn install() {
    normals::install();
    smashes::install();
    aerials::install();
    catch::install();
    throws::install();
    specials::install();
    glide::install();
}