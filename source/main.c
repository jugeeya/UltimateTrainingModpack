#include <switch.h>

#include <string.h>
#include <stdio.h>
#include <dirent.h>
#include <sys/iosupport.h>
#include <sys/reent.h>
#include <switch/kernel/ipc.h>

#include "useful.h"

#include "saltysd_core.h"
#include "saltysd_ipc.h"
#include "saltysd_dynamic.h"

#include "lua/lua.h"
#include "lua/lstate.h"

#include "l2c.h"
#include "saltysd_helper.h"
#include "l2c_imports.h"
#include "acmd_imports.h"

u32 __nx_applet_type = AppletType_None;

static char g_heap[0x8000];

Handle orig_main_thread;
void *orig_ctx;
void *orig_saved_lr;

void (*AttackModule_clear_all_orig)(__int64_t);

void __libnx_init(void *ctx, Handle main_thread, void *saved_lr)
{
    extern char *fake_heap_start;
    extern char *fake_heap_end;

    fake_heap_start = &g_heap[0];
    fake_heap_end = &g_heap[sizeof g_heap];

    orig_ctx = ctx;
    orig_main_thread = main_thread;
    orig_saved_lr = saved_lr;

    // Call constructors.
    void
    __libc_init_array(void);
    __libc_init_array();
}

void __attribute__((weak)) NORETURN __libnx_exit(int rc)
{
    // Call destructors.
    void
    __libc_fini_array(void);
    __libc_fini_array();

    SaltySD_printf("SaltySD Plugin: jumping to %p\n", orig_saved_lr);

    __nx_exit(0, orig_saved_lr);
}

Vector3f id_colors[8] = {
    {1.0f, 0.0f, 0.0f},
    {0.7843f, 0.3529f, 1.0f},
    {1.0f, 0.7843f, 0.7843f},
    {0.0f, 1.0f, 0.8431f},
    {1.0f, 0.4706f, 0.0f},
    {0.7843f, 0.7059f, 0.0f},
    {0.7843f, 0.0f, 1.0f},
    {0.3765f, 0.2863f, 0.5294f},
};

#define is_training_mode _ZN3app9smashball16is_training_modeEv
extern uint64_t _ZN3app9smashball16is_training_modeEv(void) LINKABLE;

void AttackModule_clear_all_replace(__int64_t attack_module)
{
    AttackModule_clear_all_orig(attack_module);

    if (is_training_mode())
    {
        __int64_t battle_module_object_accessor = *(__int64_t *)(attack_module + 0x8);

        // Clear graphics every time we clear all hitboxes.
        __int64_t effect_module = *(__int64_t *)(battle_module_object_accessor + 0x140);
        void (*EffectModule_kill_kind)(__int64_t, __int64_t, __int64_t,
                                       __int64_t) =
            (void (*)(__int64_t, __int64_t, __int64_t, __int64_t))(*(__int64_t *)(*(__int64_t *)(effect_module) + 0xE0LL));

        Hash40 shieldEffectHash = {.hash = 0xAFAE75F05LL};
        EffectModule_kill_kind(effect_module, shieldEffectHash.hash, 0, 1);
    }
}

void lib_L2CAgent_push_color(__int64_t *l2c_agent, Vector3f color)
{
    L2CValue red = {.raw_float = color.x, .type = L2C_number};
    L2CValue green = {.raw_float = color.y, .type = L2C_number};
    L2CValue blue = {.raw_float = color.z, .type = L2C_number};

    lib_L2CAgent_push_lua_stack(l2c_agent, &red);
    lib_L2CAgent_push_lua_stack(l2c_agent, &green);
    lib_L2CAgent_push_lua_stack(l2c_agent, &blue);
}

void app_sv_animcmd_ATTACK_replace(__int64_t a1)
{
    SaltySD_printf("In attack code with lua_state ptr: %llx\n", a1);

    __int64_t v1; // x19
    uint64_t v2;  // x9
    uint64_t i;   // x8

    // Instantiate our own L2CAgent with the given lua_State
    L2CAgent l2c_agent;
    lib_L2CAgent(&l2c_agent, a1);

    // Get all necessary hitbox params
    L2CValue id, bone, damage, angle, kbg, wkb, bkb, size, x, y, z, x2, y2, z2;
    get_lua_stack(&l2c_agent, 1, &id);
    get_lua_stack(&l2c_agent, 3, &bone);
    get_lua_stack(&l2c_agent, 4, &damage);
    get_lua_stack(&l2c_agent, 5, &angle);
    get_lua_stack(&l2c_agent, 6, &kbg);
    get_lua_stack(&l2c_agent, 7, &wkb);
    get_lua_stack(&l2c_agent, 8, &bkb);
    get_lua_stack(&l2c_agent, 9, &size);
    get_lua_stack(&l2c_agent, 10, &x);
    get_lua_stack(&l2c_agent, 11, &y);
    get_lua_stack(&l2c_agent, 12, &z);
    get_lua_stack(&l2c_agent, 13, &x2);
    get_lua_stack(&l2c_agent, 14, &y2);
    get_lua_stack(&l2c_agent, 15, &z2);

    // original code: parse lua stack and call AttackModule::set_attack()
    v1 = a1;
    void (*sub_71019420D0)(__int64_t, __int64_t) = (void (*)(__int64_t, __int64_t))(IMPORT(0x71019420D0));
    sub_71019420D0(*(__int64_t *)(*(__int64_t *)(a1 - 8) + 416LL), a1);

    if (is_training_mode())
    {
        // Replace AttackModule::clear_all()
        __int64_t battle_module_object_accessor = *(__int64_t *)(*(__int64_t *)(a1 - 8) + 416LL);
        __int64_t attack_module = *(__int64_t *)(battle_module_object_accessor + 0xA0);
        __int64_t attack_module_clear_all = *(__int64_t *)(attack_module) + 0x50LL;
        if (AttackModule_clear_all_orig == 0)
        {
            AttackModule_clear_all_orig = (void (*)(__int64_t))(*(__int64_t *)(attack_module_clear_all));
        }
        *(__int64_t *)(attack_module_clear_all) = AttackModule_clear_all_replace;

        // Generate hitbox effect
        // EFFECT_FOLLOW_COLOR(Graphic, Bone, Z, Y, X, ZRot, YRot, XRot, Size, unknown=0x1, Red, Green, Blue)
        float sizeMult = 19.0 / 200.0;
        Hash40 shieldEffectHash = {.hash = 0xAFAE75F05LL};

        L2CValue shieldEffect = {.raw = shieldEffectHash.hash, .type = L2C_hash};
        L2CValue xRot = {.raw_float = (float)0.0, .type = L2C_number};
        L2CValue yRot = {.raw_float = (float)0.0, .type = L2C_number};
        L2CValue zRot = {.raw_float = (float)0.0, .type = L2C_number};
        L2CValue terminate = {.raw = (int)1, .type = L2C_integer};
        L2CValue attribute = {.raw = 0x101C000, .type = L2C_integer}; // for EFFECT_ATTR
        L2CValue l2c_true = {.raw = (bool)1, .type = L2C_bool};
        L2CValue l2c_false = {.raw = (bool)0, .type = L2C_bool};
        L2CValue effectSize = {.raw_float = (float)size.raw_float * sizeMult, .type = L2C_number};

        L2CValue rate = {.raw_float = 8.0f, .type = L2C_number};

        // Extended Hitboxes if x2, y2, z2 are not L2CValue::nil
        int num_effects;
        if (x2.type != L2C_void && y2.type != L2C_void && z2.type != L2C_void)
        {
            num_effects = 4;
        }
        else
        {
            x2 = x;
            y2 = y;
            z2 = z;
            num_effects = 1;
        }

        for (int i = 0; i < num_effects; i++)
        {
            L2CValue currX =
                {.raw_float = (float)x.raw_float + ((x2.raw_float - x.raw_float) / 3 * i), .type = L2C_number};
            L2CValue currY =
                {.raw_float = (float)y.raw_float + ((y2.raw_float - y.raw_float) / 3 * i), .type = L2C_number};
            L2CValue currZ =
                {.raw_float = (float)z.raw_float + ((z2.raw_float - z.raw_float) / 3 * i), .type = L2C_number};

            lib_L2CAgent_clear_lua_stack(&l2c_agent);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &shieldEffect);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &bone);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &currX);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &currY);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &currZ);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &xRot);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &yRot);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &zRot);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &effectSize);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &terminate);
            app_sv_animcmd_EFFECT_FOLLOW_NO_SCALE(l2c_agent.lua_state_agent);

            // Set to hitbox ID color
            lib_L2CAgent_clear_lua_stack(&l2c_agent);
            lib_L2CAgent_push_color(&l2c_agent, id_colors[id.raw % 8]);
            app_sv_animcmd_LAST_EFFECT_SET_COLOR(l2c_agent.lua_state_agent);

            // Speed up animation by rate to remove pulsing effect
            lib_L2CAgent_clear_lua_stack(&l2c_agent);
            lib_L2CAgent_push_lua_stack(&l2c_agent, &rate);
            app_sv_animcmd_LAST_EFFECT_SET_RATE(l2c_agent.lua_state_agent);
        }
    }

    // original code: clear_lua_stack section
    v2 = *(__int64_t *)(v1 + 16);
    for (i = **(__int64_t **)(v1 + 32) + 16LL; v2 < i; v2 = *(__int64_t *)(v1 + 16))
    {
        *(__int64_t *)(v1 + 16) = v2 + 16;
        *(__int32_t *)(v2 + 8) = 0;
    }
    *(__int64_t *)(v1 + 16) = i;
}

int main(int argc, char *argv[])
{
    SaltySD_printf("SaltySD Plugin: alive\n");

    // Get anchor for imports
    ANCHOR_ABS = SaltySDCore_getCodeStart();

    char *ver = "Ver. %d.%d.%d";
    u64 dst_3 = SaltySDCore_findCode(ver, strlen(ver));
    if (dst_3)
    {
        SaltySD_Memcpy(dst_3, "Noice v%d%d%d", 13);
    }

    // Install animCMD function replacement
    SaltySD_function_replace_sym("_ZN3app10sv_animcmd6ATTACKEP9lua_State",
                                 &app_sv_animcmd_ATTACK_replace);

    __libnx_exit(0);
}
