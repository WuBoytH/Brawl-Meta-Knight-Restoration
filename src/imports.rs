pub mod acmd_imports {
    pub use {
        smash::{
            lua2cpp::*,
            hash40,
            phx::*,
            app::{lua_bind::*, sv_animcmd::*, *},
            lib::lua_const::*
        },
        smash_script::*,
        smashline::*
    };
}

pub mod status_imports {
    pub use {
        smash::{
            lua2cpp::*,
            hash40,
            phx::*,
            app::{lua_bind::*, *},
            lib::{lua_const::*, L2CValue, L2CAgent}
        },
        smash_script::*,
        smashline::*,
        crate::{table_const::*, utility::*}
    };
}