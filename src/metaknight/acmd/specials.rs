use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "metaknight", script = "game_specialnspin", category = ACMD_GAME )]
unsafe fn metaknight_specialnspin(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 90, 100, 50, 0, 5.0, 0.0, -2.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 90, 0, 5.0, 0.0, -2.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);    
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 30, 45, 90, 0, 5.0, 0.0, 7.0, -9.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 150, 45, 90, 0, 5.0, 0.0, 7.0, 9.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialnend", category = ACMD_GAME )]
unsafe fn metaknight_specialnend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 170, 0, 35, 11.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialairnend", category = ACMD_GAME )]
unsafe fn metaknight_specialairnend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 170, 0, 35, 11.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialsdrill", category = ACMD_GAME )]
unsafe fn metaknight_specialsdrill(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 363, 50, 45, 60, 4.0, 0.0, 14.0, 4.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 367, 50, 30, 100, 3.5, 0.0, 2.0, 4.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 1.0, 367, 50, 0, 25, 5.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 1.0, 365, 50, 0, 25, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialsend", category = ACMD_GAME )]
unsafe fn metaknight_specialsend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 70, 200, 0, 30, 4.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.0, 361, 200, 0, 30, 7.0, 0.0, -8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialairsend", category = ACMD_GAME )]
unsafe fn metaknight_specialairsend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, -1.0, 2.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialairsfinish", category = ACMD_GAME )]
unsafe fn metaknight_specialairsfinish(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 70, 200, 0, 30, 4.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 3.0, 361, 200, 0, 30, 7.0, 0.0, -8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::SET_SPEED_EX(fighter, -1.0, 2.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialhi", category = ACMD_GAME )]
unsafe fn metaknight_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 70, 50, 0, 90, 6.8, 0.0, 12.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 80, 50, 0, 90, 6.8, 0.0, 0.0, 12.0, Some(0.0), Some(0.0), Some(12.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 80, 30, 0, 70, 6.0, 0.0, 14.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 80, 30, 0, 70, 6.0, 0.0, 2.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, 2, false);
        AttackModule::clear(fighter.module_accessor, 3, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        GroundModule::set_passable_check(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("body"), 6.0, 45, 100, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        GroundModule::set_passable_check(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
}

#[acmd_script( agent = "metaknight", script = "effect_specialhi", category = ACMD_EFFECT )]
unsafe fn metaknight_specialhi_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialhiloop", category = ACMD_GAME )]
unsafe fn metaknight_specialhiloop(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        macros::ATTACK(fighter, 0, 0, Hash40::new("body"), 9.0, 30, 60, 0, 90, 7.0, 0.0, 0.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 30, 60, 0, 90, 7.0, 0.0, 4.0, 4.0, Some(0.0), Some(20.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, 0, 0, Hash40::new("body"), 5.0, 361, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.0, 361, 80, 0, 30, 3.0, 0.0, 4.0, 4.0, Some(0.0), Some(20.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        GroundModule::set_passable_check(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        GroundModule::set_passable_check(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
}

#[acmd_script( agent = "metaknight", script = "effect_specialhiloop", category = ACMD_EFFECT )]
unsafe fn metaknight_specialhiloop_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, -5, 2.5, 4, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("metaknight_sword"), false, false);
    }
}

#[acmd_script( agent = "metaknight", script = "game_speciallwsubf", category = ACMD_GAME )]
unsafe fn metaknight_speciallwsubf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 150, 95, 0, 40, 5.0, 0.0, 6.4, 14.0, Some(0.0), Some(6.4), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
}

#[acmd_script( agent = "metaknight", script = "game_speciallwsubb", category = ACMD_GAME )]
unsafe fn metaknight_speciallwsubb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 5.0, 0.0, 6.4, -14.0, Some(0.0), Some(6.4), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "metaknight", script = "game_speciallwsubairf", category = ACMD_GAME )]
unsafe fn metaknight_speciallwsubairf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 120, 95, 0, 40, 5.0, 0.0, 6.4, 20.0, Some(0.0), Some(6.4), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
}

#[acmd_script( agent = "metaknight", script = "game_speciallwsubairb", category = ACMD_GAME )]
unsafe fn metaknight_speciallwsubairb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 5.0, 0.0, 6.4, -20.0, Some(0.0), Some(6.4), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "metaknight", script = "game_speciallw", category = ACMD_GAME )]
unsafe fn metaknight_speciallw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
        if sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent) {
            if macros::is_excute(fighter) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_air_lw"), false, -1.0);
            }
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 5.0, 0.0, 6.4, -14.0, Some(0.0), Some(6.4), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialairlw", category = ACMD_GAME )]
unsafe fn metaknight_specialairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
        if sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent) {
            if macros::is_excute(fighter) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_air_lw"), false, -1.0);
            }
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 5.0, 0.0, 6.4, -20.0, Some(0.0), Some(6.4), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "metaknight", script = "game_speciallwf", category = ACMD_GAME )]
unsafe fn metaknight_speciallwf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
        if sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent) {
            if macros::is_excute(fighter) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_f"), false, -1.0);
            }
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 50, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 120, 95, 0, 50, 5.0, 0.0, 6.4, 16.0, Some(0.0), Some(6.4), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialairlwf", category = ACMD_GAME )]
unsafe fn metaknight_specialairlwf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
        if sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent) {
            if macros::is_excute(fighter) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_air_lw_f"), false, -1.0);
            }
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 5.0, 0.0, 6.4, 20.0, Some(0.0), Some(6.4), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "metaknight", script = "game_speciallwb", category = ACMD_GAME )]
unsafe fn metaknight_speciallwb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
        if sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent) {
            if macros::is_excute(fighter) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_b"), false, -1.0);
            }
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 5.0, 0.0, 6.4, 14.0, Some(0.0), Some(6.4), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "metaknight", script = "game_specialairlwb", category = ACMD_GAME )]
unsafe fn metaknight_specialairlwb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
        if sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent) {
            if macros::is_excute(fighter) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_b"), false, -1.0);
            }
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 7.0, 0.0, 6.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 60, 95, 0, 40, 5.0, 0.0, 6.4, -20.0, Some(0.0), Some(6.4), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        metaknight_specialnspin,
        metaknight_specialnend,
        metaknight_specialairnend,
        metaknight_specialsdrill,
        metaknight_specialsend,
        metaknight_specialairsend,
        metaknight_specialairsfinish,
        metaknight_specialhi, metaknight_specialhi_eff,
        metaknight_specialhiloop, metaknight_specialhiloop_eff,
        metaknight_speciallwsubf,
        metaknight_speciallwsubb,
        metaknight_speciallwsubairf,
        metaknight_speciallwsubairb,
        metaknight_speciallw,
        metaknight_specialairlw,
        metaknight_speciallwf,
        metaknight_specialairlwf,
        metaknight_speciallwb,
        metaknight_specialairlwb
    );
}
