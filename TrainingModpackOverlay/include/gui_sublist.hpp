#pragma once

#include <list>
#include <tesla.hpp>
#include "clickable_list_item.hpp"

class GuiSublist : public tsl::Gui
{
private:
	std::vector<tsl::elm::ListItem*> listItems;
	std::vector<std::string>         menuItems;
	int*                             index;
	std::string                      extData;
	std::string                      title;
	std::string                      help;

public:
	GuiSublist(std::vector<std::string> menuItems, int* index, std::string extData, std::string title, std::string help);
	~GuiSublist();

	virtual tsl::elm::Element* createUI();
	virtual void               update() override;
	void                       applyChanges();
	virtual void               setClickListener(ClickableListItem* item);
};

class GuiLambda : public tsl::Gui
{
	std::function<tsl::elm::Element*()> m_createUI;
	tsl::Gui*                            m_guiMain;

public:
	virtual tsl::elm::Element* createUI() override
	{
		if(m_createUI) return m_createUI();
		return nullptr;
	}
	virtual void update() override
	{
		m_guiMain->update();
	}
	GuiLambda(std::function<tsl::elm::Element*()> createUIFunc, tsl::Gui* guiMain) : m_createUI(std::move(createUIFunc)), m_guiMain(guiMain) {}
};