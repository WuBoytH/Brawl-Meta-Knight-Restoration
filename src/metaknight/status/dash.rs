use crate::imports::status_imports::*;

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE )]
unsafe fn metaknight_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rand = sv_math::rand(hash40("fighter"), 100);
    if rand == 0 {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_SLIP);
        return 1.into();
    }
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        metaknight_dash_pre
    );
}