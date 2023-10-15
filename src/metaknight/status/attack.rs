use crate::imports::status_imports::*;

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_ATTACK_100, condition = LUA_SCRIPT_STATUS_FUNC_CHECK_ATTACK )]
unsafe fn metaknight_attack_100_check_attack(_fighter: &mut L2CFighterCommon, _param_1: L2CValue, _param_2: L2CValue) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        metaknight_attack_100_check_attack
    );
}
