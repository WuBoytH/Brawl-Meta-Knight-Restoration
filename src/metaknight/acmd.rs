mod normals;
mod smashes;
mod aerials;
mod catch;
mod throws;
mod specials;
mod glide;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    catch::install(agent);
    throws::install(agent);
    specials::install(agent);
    glide::install(agent);
}