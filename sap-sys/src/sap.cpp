#include "sap-sys/include/sap.h"

std::unique_ptr<SapLocation> location_new(const char *svr_name, int dev_id)
{
	return std::make_unique<SapLocation>(svr_name, dev_id);
}