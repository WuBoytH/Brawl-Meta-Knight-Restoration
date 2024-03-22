use crate::imports::status_imports::*;

unsafe extern "C" fn metaknight_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    ret
}

unsafe extern "C" fn metaknight_special_hi_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(Main, fighter, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP)(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(
        Main,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        metaknight_special_hi_main,
    );
    agent.status(
        Main,
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP,
        metaknight_special_hi_loop_main,
    );
}
