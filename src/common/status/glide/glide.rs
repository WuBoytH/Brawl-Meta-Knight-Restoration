use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CAgent, L2CValue}
    },
    smashline::*,
    crate::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_status_Glide)]
unsafe fn status_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("glide_direction"),
        90.0,
        0.0,
        false,
        0.0,
        false,
        false
    );
    MotionModule::add_motion_partial(
        fighter.module_accessor,
        *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY,
        Hash40::new("glide_wing"),
        0.0,
        1.0,
        false,
        false,
        0.0,
        false,
        true,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Glide_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_Glide_Main)]
unsafe fn status_glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let frame = MotionModule::frame(fighter.module_accessor);
        let min_air_frame = 60.0; // 0x13f72f238b - THIS IS AN ASSUMPTION
        if min_air_frame <= frame {
            let speed_length = KineticModule::get_sum_speed_length(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let glide_landing_speed = 2.0; // 0x134df1e1b0 - THIS IS AN ASSUMPTION
            if glide_landing_speed <= speed_length {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), false.into());
            }
        }
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
            let clear_buffer = !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), clear_buffer.into());
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
            return 0.into();
        }
    }

    0.into()
}

#[common_status_script( status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn status_glide_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let frame = MotionModule::frame(fighter.module_accessor);
        let min_air_frame = 60.0; // 0x13f72f238b - THIS IS AN ASSUMPTION
        if min_air_frame <= frame {
            let speed_length = KineticModule::get_sum_speed_length(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let glide_landing_speed = 2.0; // 0x134df1e1b0 - THIS IS AN ASSUMPTION
            if glide_landing_speed <= speed_length {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), false.into());
            }
        }
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
            let clear_buffer = !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), clear_buffer.into());
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
            return 0.into();
        }
    }

    0.into()
}

// fn get_gliding_params(kind: i32) -> GlideParams {

// }

// pub struct GlideParams {
//     angle_hi_max: f32,
//     angle_lw_max: f32,
//     v_speed_init_add: f32,
//     accel_y_mul: f32,
//     h_speed_init: f32,
//     speed_per_degree: f32,
//     speed_max: f32,
//     speed_min: f32,
//     v_speed_gravity: f32,
//     v_speed_max: f32,
// }

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Glide)]
unsafe fn bind_address_call_status_end_glide(fighter: &mut L2CFighterCommon, _param_1: L2CAgent) -> L2CValue {
    fighter.status_end_Glide()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Glide)]
unsafe fn status_end_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, false);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_glide,
            status_glide_main,
            bind_address_call_status_end_glide,
            status_end_glide
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    // install_status_scripts!(
    //     status_glide_exec
    // );
}