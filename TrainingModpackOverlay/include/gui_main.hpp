#pragma once

#include <list>
#include <tesla.hpp>
#include "value_list_item.hpp"

class GuiMain : public tsl::Gui
{
private:
	FsFileSystem                m_fs;
	tsl::elm::ToggleListItem*   toggleItem = nullptr;
	std::vector<ValueListItem*> valueListItems;

public:
	GuiMain();
	~GuiMain();

	virtual tsl::elm::Element* createUI();
	virtual void               update() override;
	void                       applyChanges();
};