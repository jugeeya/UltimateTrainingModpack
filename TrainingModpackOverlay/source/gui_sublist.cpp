#include "gui_sublist.hpp"
#include "gui_help.hpp"
#include "clickable_list_item.hpp"
#include "taunt_toggles.hpp"

GuiSublist::GuiSublist(std::vector<std::string> menuItems, int* index, std::string extData, std::string title, std::string help)
{
	this->menuItems = menuItems;
	this->index     = index;
	this->extData   = extData;
	this->title     = title;
	this->help      = help;
}

GuiSublist::~GuiSublist() {}

tsl::elm::Element* GuiSublist::createUI()
{
	tsl::elm::OverlayFrame* rootFrame = new tsl::elm::OverlayFrame(title, "Press \uE0E3 for help with these options.");

	auto list = new tsl::elm::List();

	for(size_t i = 0; i < menuItems.size(); i++)
	{
		auto item = new ClickableListItem(menuItems[i], menuItems, this->index, "", i, title, help);
		setClickListener(item);
		item->setHelpListener([](std::string title, std::string help) { tsl::changeTo<GuiHelp>(title, help); });
		list->addItem(item);
		listItems.push_back(item);
	}

	list->setFocusedIndex(*index);

	rootFrame->setContent(list);

	return rootFrame;
}

void GuiSublist::setClickListener(ClickableListItem* item)
{
	item->setClickListener([](std::vector<std::string> values,
	                          int*                     curValue,
	                          std::string              extdata,
	                          int                      index,
	                          std::string              title,
	                          std::string              help) {
		*curValue = index;
		tsl::goBack();
	});
}

void GuiSublist::update()
{
	static u32 counter = 0;

	if(counter++ % 15 != 0) return;

	applyChanges();
}

void GuiSublist::applyChanges() {}