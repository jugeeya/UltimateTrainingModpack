#pragma once

#include "lua_bind/MotionModule.hpp"
#include "lua_bind/ControlModule.hpp"
#include "lua_bind/CancelModule.hpp"
#include "lua_bind/EffectModule.hpp"
#include "lua_bind/WorkModule.hpp"
#include "lua_bind/StatusModule.hpp"
#include "lua_bind/KineticModule.hpp"
#include "lua_bind/JostleModule.hpp"
#include "lua_bind/GroundModule.hpp"
#include "lua_bind/GrabModule.hpp"
#include "lua_bind/DamageModule.hpp"
#include "lua_bind/CatchModule.hpp"
#include "lua_bind/CaptureModule.hpp"
#include "lua_bind/PostureModule.hpp"
#include "lua_bind/ArticleModule.hpp"
#include "lua_bind/ColorBlendModule.hpp"
#include "lua_bind/SoundModule.hpp"
#include "lua_bind/StopModule.hpp"
#include "lua_bind/ShakeModule.hpp"
#include "lua_bind/ShieldModule.hpp"
#include "lua_bind/SlopeModule.hpp"
#include "lua_bind/SlopeModule.hpp"
#include "lua_bind/ShadowModule.hpp"
#include "lua_bind/SlowModule.hpp"
#include "lua_bind/TurnModule.hpp"
#include "lua_bind/VisibilityModule.hpp"
#include "lua_bind/TeamModule.hpp"
#include "lua_bind/SearchModule.hpp"
#include "lua_bind/ReflectorModule.hpp"
#include "lua_bind/ReflectModule.hpp"
#include "lua_bind/PhysicsModule.hpp"
#include "lua_bind/MotionAnimcmdModule.hpp"
#include "lua_bind/ModelModule.hpp"
#include "lua_bind/ItemModule.hpp"
#include "lua_bind/InkPaintModule.hpp"
#include "lua_bind/HitModule.hpp"
#include "lua_bind/ComboModule.hpp"
#include "lua_bind/CameraModule.hpp"
#include "lua_bind/AttackModule.hpp"
#include "lua_bind/AreaModule.hpp"
#include "lua_bind/AbsorberModule.hpp"

#include "lua_bind/FighterWorkModuleImpl.hpp"
#include "lua_bind/FighterStopModuleImpl.hpp"
#include "lua_bind/FighterStatusModuleImpl.hpp"
#include "lua_bind/FighterMotionModuleImpl.hpp"
#include "lua_bind/FighterControlModuleImpl.hpp"
#include "lua_bind/FighterAreaModuleImpl.hpp"

namespace app::lua_bind {
	namespace FighterManager {
		u64 get_fighter_information(u64, int) asm("_ZN3app8lua_bind44FighterManager__get_fighter_information_implEPNS_14FighterManagerENS_14FighterEntryIDE") LINKABLE;
	}

	namespace FighterInformation {
		bool is_operation_cpu(u64) asm("_ZN3app8lua_bind41FighterInformation__is_operation_cpu_implEPNS_18FighterInformationE") LINKABLE;
	}
}