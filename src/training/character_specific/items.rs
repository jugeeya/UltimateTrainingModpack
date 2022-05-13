use crate::common::consts::*;
use crate::common::*;
use crate::training::save_states;
use crate::training::save_states::save_states;
use smash::app;
use smash::app::lua_bind::*;
use smash::app::ItemKind;
use smash::app::{utility, BattleObjectModuleAccessor};
use smash::cpp::l2c_value::LuaConst;
use smash::lib::lua_const::*;

pub struct CharItem {
    pub fighter_kind: LuaConst,
    pub item_kind: Option<LuaConst>,
    pub article_kind: Option<LuaConst>,
    pub variation: Option<LuaConst>,
}

pub const ALL_CHAR_ITEMS: [CharItem; 45] = [
    CharItem {
        fighter_kind: FIGHTER_KIND_DIDDY,
        item_kind: None,
        article_kind: Some(FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA),
        variation: None,
    },
    CharItem {
        // Robin Tome
        fighter_kind: FIGHTER_KIND_REFLET,
        item_kind: Some(ITEM_KIND_BOOK),
        article_kind: None,
        variation: None, // TODO: Look at the lua const ITEM_BOOK_STATUS_KIND_BEFORE_BORN
    },
    CharItem {
        // Banjo-Kazooie Grenade Egg
        fighter_kind: FIGHTER_KIND_BUDDY,
        item_kind: Some(ITEM_KIND_BUDDYBOMB),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: None,
        article_kind: Some(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_DAISYDAIKON_1), // Smile
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: None,
        article_kind: Some(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_DAISYDAIKON_6), // Winky
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: None,
        article_kind: Some(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_DAISYDAIKON_7), // Dot-Eyes
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: None,
        article_kind: Some(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_DAISYDAIKON_8), // Stitch-face
    },
    CharItem {
        // Mr Saturn
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: Some(ITEM_KIND_DOSEISAN),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Bob-omb
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: Some(ITEM_KIND_BOMBHEI),
        article_kind: None,
        variation: Some(ITEM_VARIATION_BOMBHEI_NORMAL),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_DIDDY,
        item_kind: Some(ITEM_KIND_DIDDYPEANUTS),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Sheik Sideb Bomb
        fighter_kind: FIGHTER_KIND_SHEIK,
        item_kind: Some(ITEM_KIND_EXPLOSIONBOMB),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_KROOL,
        item_kind: Some(ITEM_KIND_KROOLCROWN),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_LINK,
        item_kind: Some(ITEM_KIND_LINKARROW),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_LINK,
        item_kind: Some(ITEM_KIND_LINKBOMB),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_KOOPAJR,
        item_kind: Some(ITEM_KIND_MECHAKOOPA),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROCKMAN,
        item_kind: Some(ITEM_KIND_METALBLADE),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANCHERRY),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANSTRAWBERRY),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANORANGE),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANAPPLE),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANMELON),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANBOSS),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANBELL),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: Some(ITEM_KIND_PACMANKEY),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: None,
        article_kind: Some(FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_PEACHDAIKON_1), // Smile
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: None,
        article_kind: Some(FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_PEACHDAIKON_6), // Winky
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: None,
        article_kind: Some(FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_PEACHDAIKON_7), // Dot-Eyes
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: None,
        article_kind: Some(FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON),
        variation: Some(ITEM_VARIATION_PEACHDAIKON_8), // Stitch-face
    },
    CharItem {
        // Mr Saturn
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: Some(ITEM_KIND_DOSEISAN),
        article_kind: None,
        variation: None,
    },
    CharItem {
        // Bob-omb
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: Some(ITEM_KIND_BOMBHEI),
        article_kind: None,
        variation: Some(ITEM_VARIATION_BOMBHEI_NORMAL),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_RICHTER,
        item_kind: Some(ITEM_KIND_RICHTERHOLYWATER),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_1P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_2P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_3P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_4P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_5P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_6P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_7P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: Some(ITEM_KIND_ROBOTGYRO),
        article_kind: None,
        variation: Some(ITEM_VARIATION_ROBOTGYRO_8P),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_SIMON,
        item_kind: Some(ITEM_KIND_SIMONHOLYWATER),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_SNAKE,
        item_kind: Some(ITEM_KIND_SNAKEGRENADE),
        article_kind: None,
        variation: None,
    },
    // CharItem {
    //     // Cardboard Box from Taunt
    //     fighter_kind: FIGHTER_KIND_SNAKE,
    //     item_kind: Some(ITEM_KIND_SNAKECBOX),
    //     article_kind: None,
    //     variation: None,
    // },
    CharItem {
        // Robin Levin Sword
        fighter_kind: FIGHTER_KIND_REFLET,
        item_kind: Some(ITEM_KIND_THUNDERSWORD),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_TOONLINK,
        item_kind: Some(ITEM_KIND_TOONLINKBOMB),
        article_kind: None,
        variation: None,
    },
    // CharItem {
    //     fighter_kind: FIGHTER_KIND_WARIO,
    //     item_kind: Some(ITEM_KIND_WARIOBIKE),
    //     // Pretty sure these other ones are just the bike parts
    //     // ITEM_KIND_WARIOBIKEA,
    //     // ITEM_KIND_WARIOBIKEB,
    //     // ITEM_KIND_WARIOBIKEC,
    //     // ITEM_KIND_WARIOBIKED,
    //     // ITEM_KIND_WARIOBIKEE,
    //     article_kind: None,
    //     variation: None,
    // },
    CharItem {
        // Villager Wood Chip
        fighter_kind: FIGHTER_KIND_MURABITO,
        item_kind: Some(ITEM_KIND_WOOD),
        article_kind: None,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_YOUNGLINK,
        item_kind: Some(ITEM_KIND_YOUNGLINKBOMB),
        article_kind: None,
        variation: None,
    },
];

pub static mut TURNIP_CHOSEN: Option<u32> = None;

pub unsafe fn apply_item(
    module_accessor: &mut BattleObjectModuleAccessor,
    fighter_kind: i32,
    character_item: CharacterItem,
) {
    let variation_idx = (character_item as i32 - 1) as usize;
    ALL_CHAR_ITEMS
        .iter()
        .filter(|item| item.fighter_kind == fighter_kind)
        .nth(variation_idx)
        .map(|item| {
            let variation = item.variation.as_ref().map(|v| **v).unwrap_or(0);
            item.item_kind.as_ref().map(|item_kind| {
                let item_kind = **item_kind;
                if item_kind == *ITEM_KIND_LINKBOMB {
                    WorkModule::on_flag(
                        module_accessor,
                        *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB,
                    );
                } else {
                    ItemModule::have_item(
                        module_accessor,
                        smash::app::ItemKind(item_kind),
                        variation,
                        0,
                        false,
                        false,
                    );
                }
            });
            item.article_kind.as_ref().map(|article_kind| {
                TURNIP_CHOSEN = if [*ITEM_VARIATION_PEACHDAIKON_8, *ITEM_VARIATION_DAISYDAIKON_8]
                    .contains(&variation)
                {
                    Some(8)
                } else if [*ITEM_VARIATION_PEACHDAIKON_7, *ITEM_VARIATION_DAISYDAIKON_7]
                    .contains(&variation)
                {
                    Some(7)
                } else if [*ITEM_VARIATION_PEACHDAIKON_6, *ITEM_VARIATION_DAISYDAIKON_6]
                    .contains(&variation)
                {
                    Some(6)
                } else if [*ITEM_VARIATION_PEACHDAIKON_1, *ITEM_VARIATION_DAISYDAIKON_1]
                    .contains(&variation)
                {
                    Some(1)
                } else {
                    None
                };
                ArticleModule::generate_article(module_accessor, **article_kind, false, 0);
                TURNIP_CHOSEN = None;
            });
        });
}

macro_rules! daikon_replace {
    ($caps_char: ident, $char:ident, $num:literal) => {
        paste::paste! {
            extern "C" {
                #[link_name = "\u{1}_ZN3app11" $char "daikon31" $caps_char "_" $caps_char "DAIKON_DAIKON_" $num "_PROBEv"]
                pub fn [<$char daikon_ $num _prob>]() -> f32;
            }

            #[skyline::hook(replace = [<$char daikon_ $num _prob>])]
            pub unsafe fn [<handle_ $char daikon_ $num _prob>]() -> f32 {
                let orig = original!()();
                if is_training_mode() {
                    if TURNIP_CHOSEN == Some($num) {
                        return 58.0;
                    } else if TURNIP_CHOSEN != None {
                        return 0.0;
                    }
                }

                orig
            }
        }
    };
}

daikon_replace!(PEACH, peach, 8);
daikon_replace!(PEACH, peach, 7);
daikon_replace!(PEACH, peach, 6);
daikon_replace!(PEACH, peach, 5);
daikon_replace!(PEACH, peach, 4);
daikon_replace!(PEACH, peach, 3);
daikon_replace!(PEACH, peach, 2);
daikon_replace!(PEACH, peach, 1);
daikon_replace!(DAISY, daisy, 8);
daikon_replace!(DAISY, daisy, 7);
daikon_replace!(DAISY, daisy, 6);
daikon_replace!(DAISY, daisy, 5);
daikon_replace!(DAISY, daisy, 4);
daikon_replace!(DAISY, daisy, 3);
daikon_replace!(DAISY, daisy, 2);
daikon_replace!(DAISY, daisy, 1);

#[skyline::hook(replace = smash::app::lua_bind::ItemManager::is_change_fighter_restart_position)]
pub unsafe fn is_change_fighter_restart_position(mgr: *mut smash::app::ItemManager) -> bool {
    let ori = original!()(mgr);
    // Remove all items when reverting to save state
    if is_training_mode() && save_states::is_killing() {
        return true;
    }

    ori
}

pub fn init() {
    skyline::install_hooks!(
        is_change_fighter_restart_position,
        handle_peachdaikon_8_prob,
        handle_peachdaikon_7_prob,
        handle_peachdaikon_6_prob,
        handle_peachdaikon_5_prob,
        handle_peachdaikon_4_prob,
        handle_peachdaikon_3_prob,
        handle_peachdaikon_2_prob,
        handle_peachdaikon_1_prob,
        handle_daisydaikon_8_prob,
        handle_daisydaikon_7_prob,
        handle_daisydaikon_6_prob,
        handle_daisydaikon_5_prob,
        handle_daisydaikon_4_prob,
        handle_daisydaikon_3_prob,
        handle_daisydaikon_2_prob,
        handle_daisydaikon_1_prob,
    );
}
