use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn metaknight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if [
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FLY
        ].contains(&fighter.global_table[0xB].get_i32()) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_AVAILABLE_GLIDE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        metaknight_frame
    );
}
