
#pragma once

#include <tesla.hpp>

class ClickableListItem : public tsl::elm::ListItem
{
public:
	ClickableListItem(std::string                    text,
	                  const std::vector<std::string> values,
	                  int*                           defaultPos,
	                  const std::string              data,
	                  int                            index,
	                  std::string                    title,
	                  std::string                    help)
	    : tsl::elm::ListItem(text), m_values(values), m_curValue(defaultPos), extdata(data), title(title), help(help)
	{
		this->index = index;
		this->setValue("");
	}

	~ClickableListItem() {}

	tsl::elm::Element* requestFocus(Element* oldFocus, tsl::FocusDirection direction)
	{
		return ListItem::requestFocus(oldFocus, direction);
	}

	void layout(u16 parentX, u16 parentY, u16 parentWidth, u16 parentHeight)
	{
		ListItem::layout(parentX, parentY, parentWidth, parentHeight);
	}

	bool onClick(u64 keys)
	{
		if(keys & KEY_Y)
		{
			if(this->m_helpListener != nullptr)
			{
				this->m_helpListener(this->title, this->help);
				return true;
			}
		}
		if(keys & KEY_A)
		{
			if(this->m_clickListener != nullptr)
			{
				this->m_clickListener(this->m_values, this->m_curValue, this->extdata, this->index, this->title, this->help);
				return true;
			}
		}

		return false;
	}

	int  getCurValue() { return *(this->m_curValue); }
	void setCurValue(int value) { *(this->m_curValue) = value; }

	const std::string getExtData() { return this->extdata; }

	const std::vector<std::string> getValues() { return this->m_values; }

	void setClickListener(
	    std::function<void(const std::vector<std::string>, int*, std::string, int index, std::string title, std::string help)>
	        clickListener)
	{
		this->m_clickListener = clickListener;
	}
	void setHelpListener(std::function<void(std::string, std::string)> helpListener) { this->m_helpListener = helpListener; }

private:
	const std::vector<std::string>                                                                        m_values;
	int*                                                                                                  m_curValue;
	std::function<void(const std::vector<std::string>, int*, std::string, int, std::string, std::string)> m_clickListener =
	    nullptr;
	std::function<void(std::string, std::string)> m_helpListener = nullptr;
	const std::string                             extdata;
	const std::string                             title;
	const std::string                             help;
	int                                           index;
};
