#include <tesla.hpp>
#include "gui_main.hpp"
#include "gui_help.hpp"
#include "value_list_item.hpp"
#include "clickable_list_item.hpp"
#include "taunt_toggles.hpp"

static struct TrainingModpackMenu
{
	int        HITBOX_VIS      = true;
	int        DI_STATE        = NONE;
	int        LEFT_STICK      = NONE;
	int        ATTACK_STATE    = MASH_NAIR;
	int        FOLLOW_UP       = 0;
	LedgeFlags LEDGE_STATE     = LedgeFlags::All;
	TechFlags  TECH_STATE      = TechFlags::All;
	int        MASH_STATE      = NONE;
	int        SHIELD_STATE    = NONE;
	int        DEFENSIVE_STATE = RANDOM_DEFENSIVE;
	int        OOS_OFFSET      = 0;
	int        REACTION_TIME   = 0;
	int        MASH_IN_NEUTRAL = false;
	int        FAST_FALL       = false;
	int        FAST_FALL_DELAY = 0;
	int        FALLING_AERIALS = false;
	int        FULL_HOP        = false;
} menu;

static int FRAME_ADVANTAGE = 0;

u64                pidSmash                   = 0;
static const char* SYSTEM_SETTINGS_FILE       = "/atmosphere/config/system_settings.ini";
static const char* TRAINING_MOD_LOG           = "/TrainingModpack/training_modpack.log";
static const char* TRAINING_MOD_FRAME_ADV_LOG = "/TrainingModpack/training_modpack_frame_adv.log";
static const char* TRAINING_MOD_CONF          = "/TrainingModpack/training_modpack_menu.conf";

static tsl::hlp::ini::IniData readSettings()
{
	/* Open Sd card filesystem. */
	FsFileSystem fsSdmc;
	if(R_FAILED(fsOpenSdCardFileSystem(&fsSdmc))) return {};
	tsl::hlp::ScopeGuard fsGuard([&] { fsFsClose(&fsSdmc); });

	/* Open config file. */
	FsFile fileConfig;
	if(R_FAILED(fsFsOpenFile(&fsSdmc, SYSTEM_SETTINGS_FILE, FsOpenMode_Read, &fileConfig))) return {};
	tsl::hlp::ScopeGuard fileGuard([&] { fsFileClose(&fileConfig); });

	/* Get config file size. */
	s64 configFileSize;
	if(R_FAILED(fsFileGetSize(&fileConfig, &configFileSize))) return {};

	/* Read and parse config file. */
	std::string configFileData(configFileSize, '\0');
	u64         readSize;
	Result      rc = fsFileRead(&fileConfig, 0, configFileData.data(), configFileSize, FsReadOption_None, &readSize);
	if(R_FAILED(rc) || readSize != static_cast<u64>(configFileSize)) return {};

	return tsl::hlp::ini::parseIni(configFileData);
}

static void writeSettings(tsl::hlp::ini::IniData const& iniData)
{
	/* Open Sd card filesystem. */
	FsFileSystem fsSdmc;
	if(R_FAILED(fsOpenSdCardFileSystem(&fsSdmc))) return;
	tsl::hlp::ScopeGuard fsGuard([&] { fsFsClose(&fsSdmc); });

	std::string iniString = tsl::hlp::ini::unparseIni(iniData);

	fsFsDeleteFile(&fsSdmc, SYSTEM_SETTINGS_FILE);
	fsFsCreateFile(&fsSdmc, SYSTEM_SETTINGS_FILE, iniString.length(), 0);

	/* Open config file. */
	FsFile fileConfig;
	if(R_FAILED(fsFsOpenFile(&fsSdmc, SYSTEM_SETTINGS_FILE, FsOpenMode_Write, &fileConfig))) return;
	tsl::hlp::ScopeGuard fileGuard([&] { fsFileClose(&fileConfig); });

	fsFileWrite(&fileConfig, 0, iniString.c_str(), iniString.length(), FsWriteOption_Flush);
}

static void updateSettings(tsl::hlp::ini::IniData const& changes)
{
	tsl::hlp::ini::IniData iniData = readSettings();
	for(auto& section : changes)
	{
		for(auto& keyValue : section.second)
		{
			iniData[section.first][keyValue.first] = keyValue.second;
		}
	}
	writeSettings(iniData);
}

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
template<typename T> tsl::elm::ListItem* createBitFlagOption(T* option, const std::string& name, const std::string& help)
{
	using FlagType = typename T::Type;

	auto item = new tsl::elm::ListItem(name);
	item->setClickListener([name, help, option](u64 keys) -> bool {
		if(keys & KEY_A)
		{
			tsl::changeTo<GuiLambda>([option, name]() -> tsl::elm::Element* {
				auto                                   toggleList = new tsl::elm::List();
				std::vector<tsl::elm::ToggleListItem*> items;
				for(auto& [flag, str] : detail::EnumArray<FlagType>::values)
				{
					items.emplace_back(new BitFlagToggleListItem<FlagType>(str, flag, option));
				}

				auto allOff = new SetToggleListItem({}, items, "None");
				auto allOn  = new SetToggleListItem(items, {}, "All");

				toggleList->addItem(allOn);
				toggleList->addItem(allOff);

				for(auto it : items)
				{
					toggleList->addItem(it);
				}

				auto frame = new tsl::elm::OverlayFrame(name, "");
				frame->setContent(toggleList);
				return frame;
			});
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
	tsl::elm::OverlayFrame* rootFrame = new tsl::elm::OverlayFrame("Training Modpack", buffer);

	auto list = new tsl::elm::List();

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

			ValueListItem* hitboxItem =
			    new ValueListItem("Hitbox Visualization", on_off, &menu.HITBOX_VIS, "hitbox", hitbox_help);
			list->addItem(hitboxItem);
			valueListItems.push_back(hitboxItem);

			ValueListItem* shieldItem =
			    new ValueListItem("Shield Options", shield_items, &menu.SHIELD_STATE, "shield", shield_help);
			list->addItem(shieldItem);
			valueListItems.push_back(shieldItem);

			ValueListItem* mashItem = new ValueListItem("Mash Toggles", mash_items, &menu.MASH_STATE, "mash", mash_help);
			list->addItem(mashItem);
			valueListItems.push_back(mashItem);

			ValueListItem* attackItem =
			    new ValueListItem("Attack Toggles", attack_items, &menu.ATTACK_STATE, "attack", attack_help);
			list->addItem(attackItem);
			valueListItems.push_back(attackItem);

			ValueListItem* followUp =
			    new ValueListItem("Followup Toggles", action_items, &menu.FOLLOW_UP, "followUp", follow_up_help);
			list->addItem(followUp);
			valueListItems.push_back(followUp);

			ValueListItem* mashNeutralItem =
			    new ValueListItem("Mash In Neutral", on_off, &menu.MASH_IN_NEUTRAL, "mash_neutral", mash_neutral_help);
			list->addItem(mashNeutralItem);
			valueListItems.push_back(mashNeutralItem);

			list->addItem(createBitFlagOption(&menu.LEDGE_STATE, "Ledge Options", ledge_help));
			list->addItem(createBitFlagOption(&menu.TECH_STATE, "Tech Options", tech_help));

			ValueListItem* defensiveItem =
			    new ValueListItem("Defensive Options", defensive_items, &menu.DEFENSIVE_STATE, "defensive", defensive_help);
			list->addItem(defensiveItem);
			valueListItems.push_back(defensiveItem);

			ValueListItem* diItem = new ValueListItem("Set DI", di_items, &menu.DI_STATE, "di", di_help);
			list->addItem(diItem);
			valueListItems.push_back(diItem);

			ValueListItem* leftStickItem =
			    new ValueListItem("Left Stick", di_items, &menu.LEFT_STICK, "leftStick", left_stick_help);
			list->addItem(leftStickItem);
			valueListItems.push_back(leftStickItem);

			ValueListItem* oosOffsetItem = new ValueListItem("OOS Offset", number_list, &menu.OOS_OFFSET, "oos", oos_help);
			list->addItem(oosOffsetItem);
			valueListItems.push_back(oosOffsetItem);

			ValueListItem* reactionTime =
			    new ValueListItem("Reaction Time", number_list_big, &menu.REACTION_TIME, "reaction_time", reaction_time_help);
			list->addItem(reactionTime);
			valueListItems.push_back(reactionTime);

			ValueListItem* fastFallItem = new ValueListItem("Fast Fall", on_off, &menu.FAST_FALL, "fast_fall", "");
			list->addItem(fastFallItem);
			valueListItems.push_back(fastFallItem);

			ValueListItem* fastFallDelay =
			    new ValueListItem("Fast Fall Delay", number_list_big, &menu.FAST_FALL_DELAY, "fast_fall", "In Frames");
			list->addItem(fastFallDelay);
			valueListItems.push_back(fastFallDelay);

			ValueListItem* fallingAerialsItem =
			    new ValueListItem("Falling Aerials", on_off, &menu.FALLING_AERIALS, "falling_aerials", "");
			list->addItem(fallingAerialsItem);
			valueListItems.push_back(fallingAerialsItem);

			ValueListItem* fullHopItem = new ValueListItem("Full Hop", on_off, &menu.FULL_HOP, "full_hop", "");
			list->addItem(fullHopItem);
			valueListItems.push_back(fullHopItem);

			ClickableListItem* saveStateItem = new ClickableListItem(
			    "Save States", save_state_items, nullptr, "saveStates", 0, "Save States", save_states_help);
			saveStateItem->setClickListener([](std::vector<std::string> values,
			                                   int*                     curValue,
			                                   std::string              extdata,
			                                   int                      index,
			                                   std::string              title,
			                                   std::string              help) { tsl::changeTo<GuiHelp>(title, help); });
			saveStateItem->setHelpListener([](std::string title, std::string help) { tsl::changeTo<GuiHelp>(title, help); });
			list->addItem(saveStateItem);

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