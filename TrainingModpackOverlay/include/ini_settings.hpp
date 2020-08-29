#include <tesla.hpp>

static const char* SYSTEM_SETTINGS_FILE       = "/atmosphere/config/system_settings.ini";

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