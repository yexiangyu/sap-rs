#pragma once
#include "rust/cxx.h"
#include <memory>

#ifdef _WIN64
#include <stdint.h>
#else
class SapLocation
{
public:
	SapLocation(const char *svr_name, int dev_id);
};
#endif

std::unique_ptr<SapLocation> location_new(const int8_t *svr_name, int dev_id);