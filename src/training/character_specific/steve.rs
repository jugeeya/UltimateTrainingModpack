use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

#[derive(Copy, Clone)]
pub struct SteveState {
    pub mat_g1: i32,
    pub mat_wood: i32,
    pub mat_stone: i32,
    pub mat_iron: i32,
    pub mat_gold: i32,
    pub mat_redstone: i32,
    pub mat_diamond: i32,
    pub sword_mat: char, // is actually FighterPickelMaterialKind, but char is same size and works
    pub sword_durability: f32,
    pub axe_mat: char,
    pub axe_durability: f32,
    pub pick_mat: char,
    pub pick_durability: f32,
    pub shovel_mat: char,
    pub shovel_durability: f32,
}

fn is_steve(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let fighter_id;
    unsafe {
        fighter_id = app::utility::get_kind(module_accessor);
    }

    fighter_id == *FIGHTER_KIND_PICKEL
}

pub fn save_steve_state(module_accessor: &mut app::BattleObjectModuleAccessor) -> Option<SteveState> {
    // Returns None if not Steve, a SteveState if it is
    if !is_steve(module_accessor) {
        return None;
    } else {
        unsafe {
            Some(save(module_accessor)) // should return the SteveState
        } 
    }
}

pub fn load_steve_state(module_accessor: &mut app::BattleObjectModuleAccessor, steve_state: SteveState) -> bool {
    // Returns false if not Steve, true if it is and we've set the variables
    if !is_steve(module_accessor) {
        return false;
    } else {
        unsafe {
            load(module_accessor, steve_state)
        }
        return true;
    }
}

unsafe fn save(module_accessor: &mut app::BattleObjectModuleAccessor) -> SteveState {
    let mat_g1 = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GRADE_1);
    let mat_wood = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_WOOD);
    let mat_stone = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_STONE);
    let mat_iron = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON);
    let mat_gold = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GOLD);
    let mat_redstone = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_RED_STONE);
    let mat_diamond = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_DIAMOND);
    let extend_buffer_address = WorkModule::get_int64(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_EXTEND_BUFFER);
    let sword_mat = *((extend_buffer_address + (0xC * 0)) as *const char);
    let sword_durability = *(((extend_buffer_address + ((0xC * 0) + 4))) as *const f32);
    let axe_mat = *((extend_buffer_address + (0xC * 1)) as *const char);
    let axe_durability = *(((extend_buffer_address + ((0xC * 1) + 4))) as *const f32);
    let pick_mat = *((extend_buffer_address + (0xC * 2)) as *const char);
    let pick_durability = *(((extend_buffer_address + ((0xC * 2) + 4))) as *const f32);
    let shovel_mat = *((extend_buffer_address + (0xC * 3)) as *const char);
    let shovel_durability = *(((extend_buffer_address + ((0xC * 3) + 4))) as *const f32);
    
    SteveState {
        mat_g1,
        mat_wood,
        mat_stone,
        mat_iron,
        mat_gold,
        mat_redstone,
        mat_diamond,
        sword_mat,
        sword_durability,
        axe_mat,
        axe_durability,
        pick_mat,
        pick_durability,
        shovel_mat,
        shovel_durability,
    }
}

unsafe fn load(module_accessor: &mut app::BattleObjectModuleAccessor, steve_state: SteveState) {
    WorkModule::set_int(module_accessor, steve_state.mat_g1, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GRADE_1);
    WorkModule::set_int(module_accessor, steve_state.mat_wood, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_WOOD);
    WorkModule::set_int(module_accessor, steve_state.mat_stone, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_STONE);
    WorkModule::set_int(module_accessor, steve_state.mat_iron, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON);
    WorkModule::set_int(module_accessor, steve_state.mat_gold, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GOLD);
    WorkModule::set_int(module_accessor, steve_state.mat_redstone, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_RED_STONE);
    WorkModule::set_int(module_accessor, steve_state.mat_diamond, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_DIAMOND);
    
    let extend_buffer_address = WorkModule::get_int64(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_EXTEND_BUFFER);
    // We have to grab the address every time instead of saving it, because loading
    //      a state from a separate training mode instance would cause a crash

    *((extend_buffer_address + (0xC * 0)) as *mut char) = steve_state.sword_mat;
    *((extend_buffer_address + (0xC * 1)) as *mut char) = steve_state.axe_mat;
    *((extend_buffer_address + (0xC * 2)) as *mut char) = steve_state.pick_mat;
    *((extend_buffer_address + (0xC * 3)) as *mut char) = steve_state.shovel_mat;

    // Update durability

    *((extend_buffer_address + (0xC * 0) + 4) as *mut f32) = steve_state.sword_durability;
    *((extend_buffer_address + (0xC * 1) + 4) as *mut f32) = steve_state.axe_durability;
    *((extend_buffer_address + (0xC * 2) + 4) as *mut f32) = steve_state.pick_durability;
    *((extend_buffer_address + (0xC * 3) + 4) as *mut f32) = steve_state.shovel_durability;

    // Update UI meter at the bottom by subtracting the materials by 0 after setting them
    
    let mut curr_material = 0;
    while curr_material < 7 {
        app::FighterSpecializer_Pickel::sub_material_num(module_accessor,curr_material,0);
        curr_material += 1;
    }
}