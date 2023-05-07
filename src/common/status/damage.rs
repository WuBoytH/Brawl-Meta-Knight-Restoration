use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_ftStatusUniqProcessDamage_init_common)]
unsafe fn ftstatusuniqprocessdamage_init_common(fighter: &mut L2CFighterCommon) {
    let reaction_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME);
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_x"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let speed_vec_x = fighter.pop_lua_stack(1).get_f32();
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_y"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let speed_vec_y = fighter.pop_lua_stack(1).get_f32();
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("attr"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let attr = fighter.pop_lua_stack(1).get_u64();
    let _status = StatusModule::status_kind(fighter.module_accessor);
    if 0 >= reaction_frame as i32 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION);
        WorkModule::set_float(fighter.module_accessor, reaction_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::set_float(fighter.module_accessor, reaction_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("angle"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let angle = fighter.pop_lua_stack(1).get_f32();
    let degrees = angle.to_degrees();
    let fighter_kind = fighter.global_table[KIND].get_i32();
    let attacker_ids = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
    let mut hit_by_mk = false;
    for x in 0..8 {
        if attacker_ids & (1 << x) == 0 {
            continue;
        }
        if let Some(object_id) = get_active_battle_object_id_from_entry_id(x) {
            let object = get_battle_object_from_id(object_id);
            let module_accessor = (*object).module_accessor;
            let kind = utility::get_kind(&mut *module_accessor);
            if kind == *FIGHTER_KIND_METAKNIGHT {
                hit_by_mk = true;
                break;
            }
        }
    }
    if ![fighter_kind].contains(&*FIGHTER_KIND_METAKNIGHT) && !hit_by_mk {
        fighter.FighterStatusDamage_init_damage_speed_up(reaction_frame.into(), degrees.into(), false.into());
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
    }
    let damage_cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_cliff_no_catch_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
    let cursor_fly_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cursor_fly_speed"));
    let pop1squared = speed_vec_x * speed_vec_x;
    let pop2squared = speed_vec_y * speed_vec_y;
    let combined = pop1squared + pop2squared;
    let cursor_fly_speed_squared = cursor_fly_speed * cursor_fly_speed;
    if cursor_fly_speed_squared < combined {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
        let cursor_fly_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cursor_fly_frame"));
        WorkModule::set_int(fighter.module_accessor, cursor_fly_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CURSOR_FRAME);
    }
    let damage_fly_attack_frame = if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        25
    }
    else {
        WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_attack_frame"))
    };
    WorkModule::set_int(fighter.module_accessor, damage_fly_attack_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME);
    let damage_fly_escape_frame = if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        13
    }
    else {
        WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_frame"))
    };
    WorkModule::set_int(fighter.module_accessor, damage_fly_escape_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME);
    if [
        hash40("collision_attr_paralyze"),
        hash40("collision_attr_paralyze_ghost")
    ].contains(&attr) {
        let invalid_paralyze_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("invalid_paralyze_frame"));
        WorkModule::set_float(fighter.module_accessor, invalid_paralyze_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_PARALYZE_FRAME);
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_DamageFlyChkUniq)]
unsafe fn subdamageflychkuniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN) {
            if fighter.sub_AirChkDown().get_bool() {
                return true.into();
            }
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            let damage_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME);
            if -1.0 <= damage_speed_y
            && WorkModule::get_param_int(fighter.module_accessor, hash40("common"), 0x1e7a52eb8a) <=damage_frame {
                if fighter.sub_AirChkDown().get_bool() {
                    return true.into();
                }
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            let damage_speed_length = sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);
            let mk = fighter.global_table[KIND].get_i32() == *FIGHTER_KIND_METAKNIGHT;
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME) <= 0 {
                if mk
                || WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_attack_speed")) <= damage_speed_length {
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
                }
            }
            if 1.0 < fighter.global_table[MOTION_FRAME].get_f32()
            && WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME) <= 0 {
                if mk
                || WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_speed")) <= damage_speed_length {
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
                    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                }
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_DAMAGE_REFLECT_ESCAPE_DISABLE_FRAME) <= 0 {
                WorkModule::enable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            }
        }
        fighter.FighterStatusDamage__check_smoke_effect();
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            ftstatusuniqprocessdamage_init_common,
            subdamageflychkuniq
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}