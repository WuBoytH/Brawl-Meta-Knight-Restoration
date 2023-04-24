use crate::imports::status_imports::*;

#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS )]
unsafe fn metaknight_escape_air_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if [
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY
    ].contains(&prev_status) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR);
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            0.0
        );
    }
    if [
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_TREAD_FALL
    ].contains(&prev_status) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    let escape_air_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_stiff_start_frame"));
    WorkModule::set_float(fighter.module_accessor, escape_air_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
    let escape_air_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_stiff_frame"));
    WorkModule::set_float(fighter.module_accessor, escape_air_stiff_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
    let some_bool = WorkModule::get_param_int(fighter.module_accessor, 0x2ea659cf56, 0) == 1;
    if some_bool {
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        if mot == hash40("jump_aerial_f") || mot == hash40("jump_aerial_b") {
            let some_float = WorkModule::get_param_float(fighter.module_accessor, 0x271984ee09, 0);
            let some_float2 = WorkModule::get_param_float(fighter.module_accessor, 0x2bf4bef265, 0);
            let frame = MotionModule::frame(fighter.module_accessor);
            let mut result = some_float - (some_float2 * frame);
            let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
            if result < -air_speed_y_stable {
                result = -air_speed_y_stable;
            }
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                result,
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                enable,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY
            );
        }
    }
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let mut hit_xlu_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_hit_xlu_frame"));
    let hit_normal_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_hit_normal_frame"));
    if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("escape_air")) {
        hit_xlu_frame -= 1.0;
    }
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
    WorkModule::set_float(fighter.module_accessor, -1.0, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
    let escape_invincible_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("escape_invincible_frame"), 0);
    let interp_invuln = hit_normal_frame * escape_invincible_frame;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
    }
    WorkModule::set_int(fighter.module_accessor, hit_xlu_frame as i32, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
    WorkModule::set_int(fighter.module_accessor, interp_invuln as i32, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_NORMAL_FRAME);
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        let add_xlu_start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_ESCAPE_ADD_XLU_START_FRAME);
        if 0 < add_xlu_start_frame {
            WorkModule::add_int(fighter.module_accessor, add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
            WorkModule::set_int(fighter.module_accessor, add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_ESCAPE_ADD_XLU_START_FRAME);
        }
    }
    0.into()
}

// #[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
// unsafe fn metaknight_escape_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     metaknight_escape_air_common(fighter);
//     MotionModule::change_motion(
//         fighter.module_accessor,
//         Hash40::new("escape_air"),
//         0.0,
//         1.0,
//         false,
//         0.0,
//         false,
//         false
//     );
//     fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_EscapeAir_Main as *const () as _))
// }

// unsafe fn metaknight_escape_air_common(fighter: &mut L2CFighterCommon) {
//     ControlModule::reset_trigger(fighter.module_accessor);
//     WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
//     WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
//     WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
//     WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
//     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
//     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
//     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON);
//     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP);
//     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL);
//     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL);
//     WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_NO_WATER_INOUT_FRAME);
// }

pub fn install() {
    install_status_scripts!(
        metaknight_escape_air_init,
        // metaknight_escape_air_main
    );
}