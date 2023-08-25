use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::Vector3f;

#[repr(C)]
struct TroopsManager {
    _x0: u64,
    max_pikmin_count: usize, // always 3
    current_pikmin_count: usize,
    pikmin_objects: *mut *mut app::BattleObject,
    pikmin: [*mut app::BattleObject; 3], // unk
    padding_0: u64,
    padding_1: u64,
    padding_2: u64,
    padding_3: u64,
    padding_4: u64,
    padding_5: u64,
    padding_6: u64,
    padding_7: u64,
    padding_8: u64,
    held_pikmin_count: u64,
    maybe_more_pikmin_objects: *mut *mut app::BattleObject,
    held_pikmin: [*mut app::BattleObject; 3], // @ 0x90
}

#[skyline::from_offset(0x3ac540)]
fn get_battle_object_from_id(id: u32) -> *mut app::BattleObject;

fn get_pikmin_prev(variation: i32) -> i32 {
    if variation > 0 { 
        return variation - 1; 
    }
    4
}

unsafe fn rotate(module_accessor: &mut app::BattleObjectModuleAccessor, correct_order: [Option<i32>; 3]) -> bool {
    // Rotate pikmin until the correct one is in front, return true if order is correct
    let mut current_order = get_current_pikmin(module_accessor);
    if current_order == correct_order {
        return true; // Christmas Miracle
    }
    for _ in 0..2 {
        app::FighterSpecializer_Pikmin::sort_pikmin_no_change_status(module_accessor as *mut app::BattleObjectModuleAccessor as *mut app::FighterModuleAccessor);
        current_order = get_current_pikmin(module_accessor);
        if current_order == correct_order {
            return true;
        }
    }
    false
}

unsafe fn respawn_second(module_accessor: &mut app::BattleObjectModuleAccessor) {
    // Respawn the 2nd pikmin
    let second_pikmin_variation = get_current_pikmin(module_accessor)[1];
    if let Some(variation) = second_pikmin_variation {
        // Delete the pikmin
        let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;
        let following_count = (*troops_manager).current_pikmin_count;
        if 1 < following_count {
            let pikmin_delete_boid = (*((*troops_manager).pikmin[1])).battle_object_id;
            ArticleModule::remove_exist_object_id(module_accessor, pikmin_delete_boid);
            spawn_pikmin(module_accessor, variation);
        }
    }

}

pub unsafe fn order(module_accessor: &mut app::BattleObjectModuleAccessor, correct_order: [Option<i32>; 3]) {
    // Pikmin spawn at (basically) random, so we need to reorder them.
    
    // When spawning in a pikmin, it seems to get sent to the front or back of the pikmin, we can use this
    // see if we're only out of rotation
    if !rotate(module_accessor, correct_order) {
        // we can't rotate to get the correct order, so respawn the second pikmin, then rotate again
        respawn_second(module_accessor);
        rotate(module_accessor, correct_order);
    }
    return;
}

pub unsafe fn speed_up(module_accessor: &mut app::BattleObjectModuleAccessor, idx: usize) {
    // Make the pikmin follow Olimar without going through the entire pull out animation
    let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;
    let following_count = (*troops_manager).current_pikmin_count;

    // If the pikmin are held, we don't care about making them actionable since they're already in an action
    if idx < following_count {
        let following_boid = (*((*troops_manager).pikmin[idx])).battle_object_id;
        if following_boid != *BATTLE_OBJECT_ID_INVALID as u32
            && app::sv_battle_object::is_active(following_boid)
        {
            let pikmin_boma = app::sv_battle_object::module_accessor(following_boid);
            StatusModule::change_status_request(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_AIR_FOLLOW, false);
        }
    }
}

pub unsafe fn speed_up_all(module_accessor: &mut app::BattleObjectModuleAccessor) {
    // Make the pikmin follow Olimar without going through the entire pull out animation
    get_current_pikmin(module_accessor);
    app::FighterSpecializer_Pikmin::hold_pikmin(module_accessor as *mut app::BattleObjectModuleAccessor as *mut app::FighterModuleAccessor, 3);
}

pub unsafe fn speed_up_all_2(module_accessor: &mut app::BattleObjectModuleAccessor) {
    let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;
    
    let following_count = (*troops_manager).current_pikmin_count;
    let held_count = (*troops_manager).held_pikmin_count;

    let mut pikmin_boid_vec = Vec::new();
    //let mut pikmin_boma_vec: [*mut app::BattleObjectModuleAccessor; 3] = [0 as *mut app::BattleObjectModuleAccessor; 3];
    let mut ordered_pikmin_variation: [Option<i32>; 3] = [None; 3];

    // First, we get the order of held pikmin, since they'll be in front if we save state during a move or grab
    for held_index in 0..held_count {
        print!("Held index: {}", held_index);
        let held_work_var = match held_index {
            0 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0,
            1 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_1,
            2 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_2,
            _ => {panic!("Pikmin Held Out of Bounds!");}
        };
        let held_boid = WorkModule::get_int(module_accessor, held_work_var) as u32;
        println!(", boid: {}", held_boid);
        pikmin_boid_vec.push(held_boid);
    }

    // Now, we have all held pikmin boids, and want to reorder?
    for (idx, pikmin_boid) in pikmin_boid_vec.iter().enumerate() {
        if *pikmin_boid != *BATTLE_OBJECT_ID_INVALID as u32
            && app::sv_battle_object::is_active(*pikmin_boid)
        {
            let pikmin_boma = app::sv_battle_object::module_accessor(*pikmin_boid);
            let pikmin_object = get_battle_object_from_id(*pikmin_boid);
            // the below write should do nothing - these pikmin objects should already be here
            (*troops_manager).held_pikmin[idx] = pikmin_object; // troopsmanager struct very confusing, trying to hard reorder
            StatusModule::change_status_request(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_AIR_FOLLOW, false);
            let pikmin_variation = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
            println!("Index: {}, Color: {}", idx, pikmin_variation);
        }
    }
}


pub unsafe fn speed_up_all_3(module_accessor: &mut app::BattleObjectModuleAccessor) {//, correct_order: [Option<i32>; 3]) {
    // app::FighterSpecializer_Pikmin::hold_pikmin(module_accessor as *mut app::BattleObjectModuleAccessor as *mut app::FighterModuleAccessor, 1);
    // app::FighterSpecializer_Pikmin::update_hold_pikmin_param(module_accessor as *mut app::BattleObjectModuleAccessor as *mut app::FighterModuleAccessor);
    app::FighterSpecializer_Pikmin::liberty_pikmin_all(module_accessor as *mut app::BattleObjectModuleAccessor as *mut app::FighterModuleAccessor);
    //rotate(module_accessor, correct_order);
}

pub unsafe fn spawn_pikmin(module_accessor: &mut app::BattleObjectModuleAccessor, variation: i32) {
    WorkModule::set_int(module_accessor, get_pikmin_prev(variation), *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PRE_PIKMIN_VARIATION);
    WorkModule::set_int(module_accessor, get_pikmin_prev(get_pikmin_prev(variation)), *FIGHTER_PIKMIN_INSTANCE_WORK_INT_BEFORE_PRE_PIKMIN_VARIATION);
    ArticleModule::generate_article(
        module_accessor as *mut app::BattleObjectModuleAccessor,
        0,
        false,
        -1
    );
    // TODO: Try holding pikmin one at a time here, also try changing their status here?
    println!("Post-Generation: PRE: {}, BEFORE_PRE: {}", // TODO: Remove
        WorkModule::get_int(module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PRE_PIKMIN_VARIATION), 
        WorkModule::get_int(module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_BEFORE_PRE_PIKMIN_VARIATION),
    );
}

pub unsafe fn get_current_pikmin(module_accessor: &mut app::BattleObjectModuleAccessor) -> [Option<i32>; 3] {
    let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;
    
    let following_count = (*troops_manager).current_pikmin_count;
    let held_count = (*troops_manager).held_pikmin_count;

    let mut pikmin_boid_vec = Vec::new();
    //let mut pikmin_boma_vec: [*mut app::BattleObjectModuleAccessor; 3] = [0 as *mut app::BattleObjectModuleAccessor; 3];
    let mut ordered_pikmin_variation: [Option<i32>; 3] = [None; 3];

    // First, we get the order of held pikmin, since they'll be in front if we save state during a move or grab
    for held_index in 0..held_count {
        print!("Held index: {}", held_index);
        let held_work_var = match held_index {
            0 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0,
            1 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_1,
            2 => *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_2,
            _ => {panic!("Pikmin Held Out of Bounds!");}
        };
        let held_boid = WorkModule::get_int(module_accessor, held_work_var) as u32;
        println!(", boid: {}", held_boid);
        pikmin_boid_vec.push(held_boid);
    }
    // Next, we get the order of the following pikmin
    for following_index in 0..following_count {
        print!("Following index: {}", following_index);
        let following_boid = (*((*troops_manager).pikmin[following_index])).battle_object_id;
        println!(", boid: {}", following_boid);
        pikmin_boid_vec.push(following_boid);
    }
    // Now, we have all pikmin boids, and want to get their bomas (if they exist) so we can check their color
    for (idx, pikmin_boid) in pikmin_boid_vec.iter().enumerate() {
        if *pikmin_boid != *BATTLE_OBJECT_ID_INVALID as u32
            && app::sv_battle_object::is_active(*pikmin_boid)
        {
            let pikmin_boma = app::sv_battle_object::module_accessor(*pikmin_boid);
            let pikmin_variation = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
            ordered_pikmin_variation[idx] = Some(pikmin_variation);
            println!("Index: {}, Color: {}", idx, pikmin_variation);
        }
    }

    ordered_pikmin_variation
}


// pub unsafe fn speed_up_pikmin(module_accessor: &mut app::BattleObjectModuleAccessor) {
//     // Increase Motion Rate of Pikmin to have them instantly spawn in
//     // FIGHTER_PIKMIN_INSTANCE_WORK_INT_TROOPS_MANAGER_ADDRESS:
//     //let troops_manager = WorkModule::get_int(module_accessor, 0x100000C0) as *mut TroopsManager;
//     //let pikmin_1_boma = &mut *(*troops_manager).boma_pikmin_1;
//     // still crashes - unknown
//     // let troops_manager_addr = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut *mut app::BattleObjectModuleAccessor;
//     // let pikmin_1_boma_ptr_ptr = troops_manager_addr.byte_add(0x90);
//     // let pikmin_1_boma_ptr = *pikmin_1_boma_ptr_ptr;
//     // let pikmin_1_boma_ref = &mut *pikmin_1_boma_ptr;
//     //above crashes if you try to print the status of the boma - investigate

//     // let troops_manager_addr = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut *mut u32;
//     // let pikmin_1_boma_ptr_ptr = troops_manager_addr.byte_add(0x90);
//     // //let _pikmin_count_for_move = *(troops_manager_addr.byte_add(0x80) as *mut u32);
//     // //println!("pikmin_count: {}", pikmin_count);
//     // let pikmin_1_boid_ptr = (*pikmin_1_boma_ptr_ptr).byte_add(0x8);
//     // let pikmin_1_boid = *pikmin_1_boid_ptr;
//     // let pikmin_1_boma = handle_get_module_accessor(pikmin_1_boid);
//     // let pikmin_1_test = *(pikmin_1_boma.byte_add(0x8) as *mut u32);
    
//     // println!("pikmin_1_boid: {}, test: {}", pikmin_1_boid, pikmin_1_test);



//     //ArticleModule::set_float(module_accessor, 40.0, *WEAPON_PIKMIN_PIKMIN_STATUS_PULL_OUT_START_WORK_FLOAT_MOT_RATE); // can't use
//     //MotionModule::set_rate(pikmin_1_boma, 40.0); // test again
// }


/*#[derive(Copy, Clone, Debug)]
pub struct TroopsManager {
    pub unk: [i8; 0x90], // 90 bytes of unknown padding
    pub boma_pikmin_1: *mut app::BattleObjectModuleAccessor, // @ 0x90
    pub boma_pikmin_2: *mut app::BattleObjectModuleAccessor, // @ 0x98
    pub boma_pikmin_3: *mut app::BattleObjectModuleAccessor, // @ 0xa0
}*/


// unsafe fn get_current_pikmin(module_accessor: &mut app::BattleObjectModuleAccessor) -> [Option<i32>; 3] {
//     let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;
    
//     let mut pikmin_boid_array: [u32; 3] = [0; 3];
//     let mut pikmin_boma_array: [*mut app::BattleObjectModuleAccessor; 3] = [0 as *mut app::BattleObjectModuleAccessor; 3];
//     let mut ordered_pikmin_variation: [Option<i32>; 3] = [None; 3];

//     pikmin_boma_array[0] = (*troops_manager).boma_pikmin_1;
//     pikmin_boma_array[1] = (*troops_manager).boma_pikmin_2;
//     pikmin_boma_array[2] = (*troops_manager).boma_pikmin_3;

//     pikmin_boid_array[0] = *(((*troops_manager).boma_pikmin_1).byte_add(0x8) as *const u32);
//     pikmin_boid_array[1] = *(((*troops_manager).boma_pikmin_2).byte_add(0x8) as *const u32);
//     pikmin_boid_array[2] = *(((*troops_manager).boma_pikmin_3).byte_add(0x8) as *const u32);

    

//     // if pikmin_boid_array[0] != *BATTLE_OBJECT_ID_INVALID as u32 && app::sv_battle_object::is_active(pikmin_boid_array[0]) {
//     //     // pikmin is alive, set order properly
//     //     println!("Pikmin exists: {}", 0);
//     // }

//     if pikmin_boid_array[1] != *BATTLE_OBJECT_ID_INVALID as u32 && app::sv_battle_object::is_active(pikmin_boid_array[1]) { // This crashes but 0 doesn't???
//         // pikmin is alive, set order properly
//         println!("Pikmin exists: {}", 1);
//     }

//     // if pikmin_boid_array[2] != *BATTLE_OBJECT_ID_INVALID as u32 && app::sv_battle_object::is_active(pikmin_boid_array[2]) {
//     //     // pikmin is alive, set order properly
//     //     println!("Pikmin exists: {}", 2);
//     // }

//     // for index in 0..3 {
//     //     if pikmin_boid_array[index] != *BATTLE_OBJECT_ID_INVALID as u32 && app::sv_battle_object::is_active(pikmin_boid_array[index]) {
//     //         // pikmin is alive, set order properly
//     //         println!("Pikmin exists: {}", index);
//     //     }
//     // }

//     // for index in 0..3 {
//     //     if pikmin_boid_array[index] != *BATTLE_OBJECT_ID_INVALID as u32 && app::sv_battle_object::is_active(pikmin_boid_array[index]) {
//     //         // pikmin is alive, set order properly
//     //         let pikmin_boma = pikmin_boma_array[index];
//     //         let variation = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
//     //         let order_index = WorkModule::get_int(pikmin_boma, 0x1000001A) as usize; //*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_TROOPS_INDEX
//     //         if order_index < 3 {
//     //             ordered_pikmin_variation[order_index] = Some(variation);
//     //         } else {
//     //             println!("Pikmin order index out of bounds! Val: {}", order_index)
//     //         }
//     //     }
//     // }

//     ordered_pikmin_variation
// }

// unsafe fn try_pikmin_variation(module_accessor: &mut app::BattleObjectModuleAccessor, index: usize) -> Option<i32> {
//     let troops_manager = WorkModule::get_int64(module_accessor, 0x100000C0) as *mut TroopsManager;
//     let count = (*troops_manager).current_pikmin_count;
//     let pikmin;
//     let pikmin_id;
    
//     print!("Count1: {}, Count2: {}", count, (*troops_manager).held_pikmin_count);

//     if count > index {
//         pikmin = (*troops_manager).pikmin[index];
//         pikmin_id = (*pikmin).battle_object_id;
//     } else {
//         pikmin = std::ptr::null_mut();
//         pikmin_id = *BATTLE_OBJECT_ID_INVALID as u32;
//     }
//     println!(", index: {}, id: {}, 3rd held id: {}", index, pikmin_id, *(((*troops_manager).boma_pikmin_3).byte_add(0x8) as *const u32));
//     if pikmin_id != *BATTLE_OBJECT_ID_INVALID as u32
//         && app::sv_battle_object::is_active(pikmin_id)
//     {
//         return Some(WorkModule::get_int((*pikmin).module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION));
//     }
//     // If we can't access the pikmin of this index, return none
//     return None;
// }