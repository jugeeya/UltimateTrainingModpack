#include <tesla.hpp>
#include "gui_main.hpp"
#include "gui_help.hpp"
#include "overflow_list.hpp"
#include "overlay_frame_with_help.hpp"
#include "value_list_item.hpp"
#include "clickable_list_item.hpp"
#include "ini_settings.hpp"
#include "taunt_toggles.hpp"

static struct TrainingModpackMenu
{
	OnOffFlags       HITBOX_VIS          = OnOffFlag::On;
	OnOffFlags       STAGE_HAZARDS       = OnOffFlags::None;
	Directions       DI_STATE            = Directions::None;
	Directions       SDI_STATE           = Directions::None;
	int              SDI_STRENGTH        = NORMAL;
	Directions       AIR_DODGE_DIR       = Directions::None;
	ActionFlags      MASH_STATE          = ActionFlags::None;
	ActionFlags      FOLLOW_UP           = ActionFlags::None;
	AttackAngleFlags ATTACK_ANGLE        = AttackAngleFlags::None;
	LedgeFlags       LEDGE_STATE         = LedgeFlags::All;
	DelayFlags       LEDGE_DELAY         = DelayFlags::All;
	TechFlags        TECH_STATE          = TechFlags::All;
	MissTechFlags    MISS_TECH_STATE     = MissTechFlags::All;
	int              SHIELD_STATE        = NONE;
	int              SHIELD_STATE_PLAYER = NONE;
	DefensiveFlags   DEFENSIVE_STATE     = DefensiveFlags::All;
	DelayFlags       OOS_OFFSET          = DelayFlags::None;
	DelayFlags       REACTION_TIME       = DelayFlags::None;
	Directions       SHIELD_TILT         = Directions::None;
	OnOffFlags       MASH_IN_NEUTRAL     = OnOffFlags::None;
	BoolFlags        FAST_FALL           = BoolFlags::None;
	DelayFlags       FAST_FALL_DELAY     = DelayFlags::None;
	BoolFlags        FALLING_AERIALS     = BoolFlags::None;
	DelayFlags       AERIAL_DELAY        = DelayFlags::None;
	BoolFlags        FULL_HOP            = BoolFlags::None;
	int              INPUT_DELAY         = 0;
	OnOffFlags       SAVE_DAMAGE         = OnOffFlag::On;
} menu;

static struct TrainingModpackMenu defaultMenu = menu;

static int FRAME_ADVANTAGE = 0;

u64                pidSmash                   = 0;
static const char* TRAINING_MOD_LOG           = "/TrainingModpack/training_modpack.log";
static const char* TRAINING_MOD_FRAME_ADV_LOG = "/TrainingModpack/training_modpack_frame_adv.log";
static const char* TRAINING_MOD_CONF          = "/TrainingModpack/training_modpack_menu.conf";

GuiMain::GuiMain()
{
	smInitialize();
	pminfoInitialize();
	pmbmInitialize();
	smExit();

	pmdmntGetProcessId(&pidSmash, 0x01006A800016E000);

	Result rc = fsOpenSdCardFileSystem(&this->m_fs);
	if(R_FAILED(rc)) return;

	FsFile menuFile;
	rc = fsFsOpenFile(&this->m_fs, TRAINING_MOD_CONF, FsOpenMode_Read, &menuFile);
	if(R_FAILED(rc)) return;

	u64 bytesRead;
	rc = fsFileRead(&menuFile, 0, static_cast<void*>(&menu), sizeof(menu), FsReadOption_None, &bytesRead);
	if(R_FAILED(rc))
	{
		fsFileWrite(&menuFile, 0, static_cast<void*>(&menu), sizeof(menu), FsOpenMode_Write);
	}

	fsFileClose(&menuFile);
}

GuiMain::~GuiMain()
{
	// apply changes on exit
	applyChanges();

	smInitialize();
	pminfoExit();
	pmbmExit();
	smExit();
}

static char FrameAdvantage[672];

class FrameAdvantageOverlayFrame : public tsl::elm::OverlayFrame
{
public:
	FrameAdvantageOverlayFrame(const std::string& title, const std::string& subtitle) : tsl::elm::OverlayFrame(title, subtitle)
	{}

	virtual void draw(tsl::gfx::Renderer* renderer) override
	{
		renderer->clearScreen();

		renderer->drawRect(0, 0, tsl::cfg::FramebufferWidth, 85, a(tsl::style::color::ColorFrameBackground));

		renderer->drawString(this->m_title.c_str(), false, 20, 50, 30, a(tsl::style::color::ColorText));
		renderer->drawString(this->m_subtitle.c_str(), false, 20, 70, 15, a(tsl::style::color::ColorDescription));

		if(this->m_contentElement != nullptr) this->m_contentElement->frame(renderer);
	}
};

class GuiFrameAdvantage : public tsl::Gui
{
public:
	GuiFrameAdvantage()
	{
		tsl::hlp::requestForeground(false);
		smInitialize();
		pminfoInitialize();
		pmbmInitialize();
		smExit();

		pmdmntGetProcessId(&pidSmash, 0x01006A800016E000);

		Result rc = fsOpenSdCardFileSystem(&this->m_fs);
		if(R_FAILED(rc)) return;
	}

	~GuiFrameAdvantage()
	{
		smInitialize();
		pminfoExit();
		pmbmExit();
		smExit();
	}

	virtual tsl::elm::Element* createUI() override
	{
		snprintf(FrameAdvantage, 256, "Frame Advantage: %d", FRAME_ADVANTAGE);
		auto rootFrame = new FrameAdvantageOverlayFrame(FrameAdvantage, "\uE0A2 + \uE07B  Back");

		this->rootFrame = rootFrame;

		return rootFrame;
	}

	virtual void update() override
	{
		static u32 counter = 0;

		if(counter++ % 10 != 0) return;

		Result rc;
		Handle debug;

		if(pidSmash != 0)
		{
			rc = svcDebugActiveProcess(&debug, pidSmash);
			if(R_SUCCEEDED(rc))
			{
				u64    frame_adv_addr = 0;
				FsFile menuAddrFile;
				rc = fsFsOpenFile(&this->m_fs, TRAINING_MOD_FRAME_ADV_LOG, FsOpenMode_Read, &menuAddrFile);
				if(R_FAILED(rc))
				{
					snprintf(FrameAdvantage, sizeof FrameAdvantage, "Failed to open file with error %d", rc);
					rootFrame->setTitle(FrameAdvantage);
					svcCloseHandle(debug);
					return;
				}

				char buffer[100];
				u64  bytesRead;
				rc = fsFileRead(&menuAddrFile, 0, buffer, 100, FsReadOption_None, &bytesRead);
				if(R_FAILED(rc))
				{
					snprintf(FrameAdvantage, sizeof FrameAdvantage, "Failed to read file with error %d", rc);
					rootFrame->setTitle(FrameAdvantage);
					svcCloseHandle(debug);
					return;
				}

				fsFileClose(&menuAddrFile);
				buffer[bytesRead] = '\0';
				frame_adv_addr    = strtoul(buffer, NULL, 16);

				if(frame_adv_addr != 0)
				{
					rc = svcReadDebugProcessMemory(&FRAME_ADVANTAGE, debug, frame_adv_addr, sizeof(int));
					snprintf(FrameAdvantage, sizeof FrameAdvantage, "Frame Advantage: %d", FRAME_ADVANTAGE);
					rootFrame->setTitle(FrameAdvantage);
				}

				svcCloseHandle(debug);
			}
		}
		else
		{
			snprintf(FrameAdvantage, sizeof FrameAdvantage, "Smash is not running.");
			rootFrame->setTitle(FrameAdvantage);
		}
	}
	virtual bool handleInput(u64              keysDown,
	                         u64              keysHeld,
	                         touchPosition    touchInput,
	                         JoystickPosition leftJoyStick,
	                         JoystickPosition rightJoyStick) override
	{
		if(keysHeld & KEY_DLEFT)
		{
			if(keysHeld & KEY_X)
			{
				tsl::goBack();
				tsl::hlp::requestForeground(true);
				return true;
			}
		}

		// intercept B inputs
		if(keysDown & KEY_B)
		{
			return true;
		}
		return false;
	}

	FrameAdvantageOverlayFrame* rootFrame;
	FsFileSystem                m_fs;
};

namespace
{
template<typename T>
tsl::elm::ListItem* createBitFlagOption(T* option, const std::string& name, const std::string& help, GuiMain* guiMain)
{
	using FlagType = typename T::Type;

	auto item = new tsl::elm::ListItem(name);
	item->setClickListener([name, help, option, guiMain](u64 keys) -> bool {
		if(keys & KEY_A)
		{
			tsl::changeTo<GuiLambda>(
			    [option, name, help]() -> tsl::elm::Element* {
				    auto                                   toggleList = new OverflowList();
				    std::vector<tsl::elm::ToggleListItem*> items;
				    for(auto& [flag, str] : detail::EnumArray<FlagType>::values)
				    {
					    items.emplace_back(new BitFlagToggleListItem<FlagType>(str, flag, option, name, help));
				    }

				    auto allOff = new SetToggleListItem({}, items, "None", name, help);
				    auto allOn  = new SetToggleListItem(items, {}, "All", name, help);

				    toggleList->addItem(allOn);
				    toggleList->addItem(allOff);

				    for(auto it : items)
				    {
					    toggleList->addItem(it);
				    }

				    auto frame = new OverlayFrameWithHelp(name, "Press \uE0E3 for help with these options.");
				    frame->setContent(toggleList);
				    return frame;
			    },
			    guiMain);
			return true;
		}
		if(keys & KEY_Y)
		{
			tsl::changeTo<GuiHelp>(name, help);
		}
		return false;
	});
	return item;
}
} // namespace

tsl::elm::Element* GuiMain::createUI()
{
	char buffer[256];
	snprintf(buffer, 256, "Version %s", VERSION);
	OverlayFrameWithHelp* rootFrame = new OverlayFrameWithHelp("Training Modpack", buffer);

	auto list = new OverflowList();

	Result rc;
	Handle debug;

	tsl::hlp::ini::IniData iniData              = readSettings();
	bool                   ease_nro_restriction = false;
	for(auto& section : iniData)
	{
		for(auto& keyValue : section.second)
		{
			if(section.first == "ro")
			{
				if(keyValue.first == "ease_nro_restriction")
				{
					ease_nro_restriction = (readSettings()["ro"]["ease_nro_restriction"] == "u8!0x1");
				}
			}
		}
	}

	if(!ease_nro_restriction)
	{
		tsl::elm::Element* iniShow = new tsl::elm::CustomDrawer([](tsl::gfx::Renderer* renderer, u16 x, u16 y, u16 w, u16 h) {
			renderer->drawString(
			    "Your config file did not have the \nproper configuration to run the \nTraining Modpack.\n\n\nIt has been automatically \nupdated.\n- atmosphere\n---- config\n-------- system_settings.ini\n\n(enable ease_nro_restriction)\n\n\nPlease reboot your Switch.",
			    false,
			    50,
			    225,
			    20,
			    tsl::Color(255, 255, 255, 255));
		});

		updateSettings({{"ro", {{"ease_nro_restriction", "u8!0x1"}}}});

		rootFrame->setContent(iniShow);
		return rootFrame;
	}

	if(pidSmash != 0)
	{
		rc = svcDebugActiveProcess(&debug, pidSmash);
		if(R_SUCCEEDED(rc))
		{
			svcCloseHandle(debug);

			// Remove because it breaks scrolling up to the bottom of the
			// menu, because CategoryHeaders can't requestFocus?
			// list->addItem(new tsl::elm::CategoryHeader("Mash", true));

			list->addItem(createBitFlagOption(&menu.MASH_STATE, "Mash Toggles", mash_help, this));
			list->addItem(createBitFlagOption(&menu.FOLLOW_UP, "Followup Toggles", follow_up_help, this));
			list->addItem(new BitFlagToggleListItem<OnOffFlags::Type>(
			    "Mash In Neutral", OnOffFlag::On, &menu.MASH_IN_NEUTRAL, "Mash In Neutral", mash_neutral_help));
			list->addItem(createBitFlagOption(&menu.ATTACK_ANGLE, "Attack Angle", attack_angle_help, this));

			list->addItem(new tsl::elm::CategoryHeader("Left Stick", true));

			list->addItem(createBitFlagOption(&menu.DI_STATE, "Set DI", di_help, this));
			list->addItem(createBitFlagOption(&menu.SDI_STATE, "Set SDI", sdi_help, this));

			ValueListItem* sdiItem =
			    new ValueListItem("SDI Strength", strength_items, &menu.SDI_STRENGTH, "SDI Strength", sdi_strength_help);
			list->addItem(sdiItem);
			valueListItems.push_back(sdiItem);

			list->addItem(createBitFlagOption(&menu.AIR_DODGE_DIR, "Airdodge Direction", air_dodge_direction_help, this));

			list->addItem(new tsl::elm::CategoryHeader("Shield", true));

			ValueListItem* shieldItem =
			    new ValueListItem("Shield Options", shield_items, &menu.SHIELD_STATE, "shield", shield_help);
			list->addItem(shieldItem);
			valueListItems.push_back(shieldItem);

			ValueListItem* shieldItemPlayer = new ValueListItem(
			    "Player Shield", shield_items, &menu.SHIELD_STATE_PLAYER, "Player Shield", shield_help_player);
			list->addItem(shieldItemPlayer);
			valueListItems.push_back(shieldItemPlayer);

			ClickableListItem* frameAdvantageItem = new ClickableListItem("Frame Advantage",
			                                                              frame_advantage_items,
			                                                              nullptr,
			                                                              "frameAdvantage",
			                                                              0,
			                                                              "Frame Advantage",
			                                                              frame_advantage_help);
			frameAdvantageItem->setClickListener([](std::vector<std::string> values,
			                                        int*                     curValue,
			                                        std::string              extdata,
			                                        int                      index,
			                                        std::string              title,
			                                        std::string              help) { tsl::changeTo<GuiFrameAdvantage>(); });
			frameAdvantageItem->setHelpListener(
			    [](std::string title, std::string help) { tsl::changeTo<GuiHelp>(title, help); });
			list->addItem(frameAdvantageItem);

			list->addItem(createBitFlagOption(&menu.OOS_OFFSET, "OOS Offset", oos_help, this));
			list->addItem(createBitFlagOption(&menu.REACTION_TIME, "Reaction Time", reaction_time_help, this));
			list->addItem(createBitFlagOption(&menu.SHIELD_TILT, "Shield Tilt", shield_tilt_help, this));

			list->addItem(new tsl::elm::CategoryHeader("Chase", true));

			list->addItem(createBitFlagOption(&menu.LEDGE_STATE, "Ledge Options", ledge_help, this));
			list->addItem(createBitFlagOption(&menu.LEDGE_DELAY, "Ledge Delay", ledge_delay_help, this));
			list->addItem(createBitFlagOption(&menu.TECH_STATE, "Tech Options", tech_help, this));
			list->addItem(createBitFlagOption(&menu.MISS_TECH_STATE, "Missed Tech Options", miss_tech_help, this));
			list->addItem(createBitFlagOption(&menu.DEFENSIVE_STATE, "Defensive Options", defensive_help, this));

			list->addItem(new tsl::elm::CategoryHeader("Aerials", true));

			list->addItem(createBitFlagOption(&menu.FAST_FALL, "Fast Fall", fast_fall_help, this));
			list->addItem(createBitFlagOption(&menu.FAST_FALL_DELAY, "Fast Fall Delay", fast_fall_delay_help, this));
			list->addItem(createBitFlagOption(&menu.FALLING_AERIALS, "Falling Aerials", falling_aerials_help, this));
			list->addItem(createBitFlagOption(&menu.AERIAL_DELAY, "Aerial Delay", aerial_delay_help, this));
			list->addItem(createBitFlagOption(&menu.FULL_HOP, "Full Hop", full_hop_help, this));

			list->addItem(new tsl::elm::CategoryHeader("Miscellaneous", true));

			list->addItem(new BitFlagToggleListItem<OnOffFlags::Type>(
			    "Hitbox Visualization", OnOffFlag::On, &menu.HITBOX_VIS, "Hitbox Visualization", hitbox_help));

			list->addItem(new BitFlagToggleListItem<OnOffFlags::Type>(
			    "Stage Hazards", OnOffFlag::On, &menu.STAGE_HAZARDS, "Stage Hazards", hazards_help));

			ClickableListItem* saveStateItem =
			    new ClickableListItem("Save States", empty_items, nullptr, "saveStates", 0, "Save States", save_states_help);
			saveStateItem->setClickListener([](std::vector<std::string> values,
			                                   int*                     curValue,
			                                   std::string              extdata,
			                                   int                      index,
			                                   std::string              title,
			                                   std::string              help) { tsl::changeTo<GuiHelp>(title, help); });
			saveStateItem->setHelpListener([](std::string title, std::string help) { tsl::changeTo<GuiHelp>(title, help); });
			list->addItem(saveStateItem);

			list->addItem(new BitFlagToggleListItem<OnOffFlags::Type>(
			    "Save Damage", OnOffFlag::On, &menu.SAVE_DAMAGE, "Save Damage", save_damage_help));

			ValueListItem* inputDelayItem =
			    new ValueListItem("Input Delay", input_delay_items, &menu.INPUT_DELAY, "inputDelay", input_delay_help);
			list->addItem(inputDelayItem);
			valueListItems.push_back(inputDelayItem);

			ClickableListItem* resetMenuItem =
			    new ClickableListItem("Reset Menu", empty_items, nullptr, "resetMenu", 0, "Reset Menu", reset_menu_help);
			resetMenuItem->setClickListener([](std::vector<std::string> values,
			                                   int*                     curValue,
			                                   std::string              extdata,
			                                   int                      index,
			                                   std::string              title,
			                                   std::string              help) {
				menu = defaultMenu;

				/* Open Sd card filesystem. */
				FsFileSystem fsSdmc;
				if(R_FAILED(fsOpenSdCardFileSystem(&fsSdmc))) return;
				tsl::hlp::ScopeGuard fsGuard([&] { fsFsClose(&fsSdmc); });

				fsFsDeleteFile(&fsSdmc, TRAINING_MOD_CONF);

				tsl::goBack();
			});
			resetMenuItem->setHelpListener([](std::string title, std::string help) { tsl::changeTo<GuiHelp>(title, help); });
			list->addItem(resetMenuItem);

			rootFrame->setContent(list);
		}
		else
		{
			tsl::elm::Element* warning =
			    new tsl::elm::CustomDrawer([](tsl::gfx::Renderer* renderer, u16 x, u16 y, u16 w, u16 h) {
				    renderer->drawString("\uE150", false, 180, 250, 90, tsl::Color(255, 255, 255, 255));
				    renderer->drawString("Could not debug process memory", false, 110, 340, 25, tsl::Color(255, 255, 255, 255));
			    });

			rootFrame->setContent(warning);
		}
	}
	else
	{
		tsl::elm::Element* warning = new tsl::elm::CustomDrawer([](tsl::gfx::Renderer* renderer, u16 x, u16 y, u16 w, u16 h) {
			renderer->drawString("\uE150", false, 180, 250, 90, tsl::Color(255, 255, 255, 255));
			renderer->drawString("Smash not running.", false, 110, 340, 25, tsl::Color(255, 255, 255, 255));
		});

		rootFrame->setContent(warning);
	}

	return rootFrame;
}

void GuiMain::update()
{
	static u32 counter = 0;

	if(counter++ % 15 != 0) return;

	applyChanges();
}

void GuiMain::applyChanges()
{
	for(ValueListItem* item : valueListItems)
	{
		item->applyChanges();
	}
	Result rc;
	Handle debug;

	if(pidSmash != 0)
	{
		rc = svcDebugActiveProcess(&debug, pidSmash);
		if(R_SUCCEEDED(rc))
		{
			u64    menu_addr = 0;
			FsFile menuAddrFile;
			rc = fsFsOpenFile(&this->m_fs, TRAINING_MOD_LOG, FsOpenMode_Read, &menuAddrFile);
			if(R_FAILED(rc))
			{
				svcCloseHandle(debug);
				return;
			}

			char buffer[100];
			u64  bytesRead;
			rc = fsFileRead(&menuAddrFile, 0, buffer, 100, FsReadOption_None, &bytesRead);
			if(R_FAILED(rc))
			{
				svcCloseHandle(debug);
				return;
			}

			fsFileClose(&menuAddrFile);
			buffer[bytesRead] = '\0';
			menu_addr         = strtoul(buffer, NULL, 16);

			if(menu_addr != 0)
			{
				rc = svcWriteDebugProcessMemory(debug, &menu, (u64)menu_addr, sizeof(menu));
			}
			svcCloseHandle(debug);
		}
	}

	FsFile menuFile;
	fsFsCreateFile(&this->m_fs, TRAINING_MOD_CONF, sizeof(menu), 0);

	rc = fsFsOpenFile(&this->m_fs, TRAINING_MOD_CONF, FsOpenMode_Write, &menuFile);
	if(R_FAILED(rc))
	{
		fsFileClose(&menuFile);
		return;
	}

	rc = fsFileWrite(&menuFile, 0, static_cast<void*>(&menu), sizeof(menu), FsOpenMode_Write);
	if(R_FAILED(rc))
	{
		fsFileClose(&menuFile);
		return;
	}

	fsFileClose(&menuFile);
}