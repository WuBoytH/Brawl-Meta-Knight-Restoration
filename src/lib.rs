#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smash::lib::{lua_const::*, L2CValue};

mod metaknight;

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_jump_aerial)]
unsafe fn sub_transition_group_check_air_jump_aerial(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_glide_check().get_bool() {
        return true.into();
    }
    original!()(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_sub_glide_stick_check_uniq)]
unsafe fn sub_glide_stick_check_uniq(fighter: &mut L2CFighterCommon) {
    let stick_x = fighter.global_table[0x1A].get_f32();
    if stick_x.abs() < 0.5 {
        return;
    }
    let mut flick_x = fighter.global_table[0x1C].get_i32() * PostureModule::lr(fighter.module_accessor) as i32;
    let mut back = *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT_BACK) {
        flick_x *= -1;
        back = *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT_BACK;
    }
    if flick_x > 0 {
        WorkModule::on_flag(fighter.module_accessor, back);
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_glide_check)]
unsafe fn sub_glide_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    let jump_button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_WORK_INT_BUTTON_ON_FRAME);
    println!("jump button on frame: {}", jump_button_on_frame);
    if jump_button_on_frame <= 15 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE) {
                if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
                    return true.into();
                }
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE) {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_START.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_air_jump_aerial,
            sub_glide_stick_check_uniq,
            sub_glide_check
        );
    }
}

#[skyline::main(name = "brawl_mk")]
pub fn main() {
    metaknight::install();
    skyline::nro::add_hook(nro_hook);
}