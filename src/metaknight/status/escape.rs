use crate::imports::status_imports::*;

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS )]
unsafe fn metaknight_escape_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        clear_speed_ex,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_DAMAGE
    );
    let mut hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_n_hit_xlu_frame"));
    let hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_n_hit_normal_frame"));
    if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("escape")) {
        hit_xlu_frame -= 1.0;
    }
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
    let escape_invincible_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("escape_invincible_frame"), 0);
    let interp_invuln = hit_normal_frame * escape_invincible_frame;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
    }
    WorkModule::set_int(fighter.module_accessor, hit_xlu_frame as i32, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
    WorkModule::set_int(fighter.module_accessor, interp_invuln as i32, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_NORMAL_FRAME);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        metaknight_escape_init
    );
}