#include <tesla.hpp>

class OverlayFrameWithHelp : public tsl::elm::OverlayFrame
{
public:
	OverlayFrameWithHelp(const std::string& title, const std::string& subtitle) : tsl::elm::OverlayFrame(title, subtitle)
	{}

	virtual void draw(tsl::gfx::Renderer* renderer) override
	{
		renderer->fillScreen(a(tsl::style::color::ColorFrameBackground));
		renderer->drawRect(tsl::cfg::FramebufferWidth - 1, 0, 1, tsl::cfg::FramebufferHeight, a(0xF222));

		renderer->drawString(this->m_title.c_str(), false, 20, 50, 30, a(tsl::style::color::ColorText));
		renderer->drawString(this->m_subtitle.c_str(), false, 20, 70, 15, a(tsl::style::color::ColorDescription));

		renderer->drawRect(15, tsl::cfg::FramebufferHeight - 73, tsl::cfg::FramebufferWidth - 30, 1, a(tsl::style::color::ColorText));

		renderer->drawString("\uE0E1  Back     \uE0E0  OK     \uE0E3  Help", false, 30, 693, 23, a(tsl::style::color::ColorText));

		if (this->m_contentElement != nullptr)
			this->m_contentElement->frame(renderer);
	}
};