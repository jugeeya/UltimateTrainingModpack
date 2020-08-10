#define TESLA_INIT_IMPL
#include "gui_main.hpp"

class TrainingModpackOverlay : public tsl::Overlay
{
public:
	TrainingModpackOverlay() {}
	~TrainingModpackOverlay() {}

	void initServices() override { pmshellInitialize(); }

	void exitServices() override { pmshellExit(); }

	std::unique_ptr<tsl::Gui> loadInitialGui() override { return std::make_unique<GuiMain>(); }
};

int main(int argc, char** argv)
{
	return tsl::loop<TrainingModpackOverlay>(argc, argv);
}