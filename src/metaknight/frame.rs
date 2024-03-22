use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

unsafe extern "C" fn frame_metaknight(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_GLIDE_LANDING,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_MISS_FOOT,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_GLIDE_ATTACK,
        *FIGHTER_STATUS_KIND_GLIDE_END
    ].contains(&status_kind) { 
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
        macros::STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
    };
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_100 {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT_CLIFF_STOP);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) {
        if MotionModule::frame(fighter.module_accessor) > 22.0 {
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_LIGHT, false);
            }
        }
    }
}        

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, frame_metaknight);
}
