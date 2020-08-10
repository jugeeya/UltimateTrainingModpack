#pragma once
#include <tesla.hpp>

static char help_text_global[1024];

class GuiHelp : public tsl::Gui
{
public:
	GuiHelp(std::string title, std::string help) : m_title(std::move(title)), m_help(std::move(help)) {}

	virtual tsl::elm::Element* createUI() override
	{
		auto rootFrame = new tsl::elm::OverlayFrame(m_title, "Help");

		snprintf(help_text_global, sizeof help_text_global, "%s", m_help.c_str());

		auto help_text = new tsl::elm::CustomDrawer([](tsl::gfx::Renderer* renderer, u16 x, u16 y, u16 w, u16 h) {
			renderer->drawString(help_text_global, false, 45, 125, 20, renderer->a(0xFFFF));
		});

		rootFrame->setContent(help_text);

		return rootFrame;
	}

	virtual void update() override {}
	virtual bool handleInput(u64              keysDown,
	                         u64              keysHeld,
	                         touchPosition    touchInput,
	                         JoystickPosition leftJoyStick,
	                         JoystickPosition rightJoyStick) override
	{
		return false;
	}

	std::string m_title;
	std::string m_help;
};