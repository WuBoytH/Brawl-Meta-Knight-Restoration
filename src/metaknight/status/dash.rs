use crate::imports::status_imports::*;

unsafe extern "C" fn metaknight_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rand = sv_math::rand(hash40("fighter"), 10000);
    if rand <= 99 {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_SLIP);
        return 1.into();
    }
    original_status(Pre, fighter, *FIGHTER_STATUS_KIND_DASH)(fighter)
}

unsafe extern "C" fn metaknight_turn_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rand = sv_math::rand(hash40("fighter"), 10000);
    if rand <= 124 {
        PostureModule::reverse_lr(fighter.module_accessor);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_SLIP);
        return 1.into();
    }
    original_status(Pre, fighter, *FIGHTER_STATUS_KIND_TURN_DASH)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_DASH, metaknight_dash_pre);
    agent.status(
        Pre,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        metaknight_turn_dash_pre,
    );
}
