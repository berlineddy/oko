
#ifndef cheddar_generated_liboko_h
#define cheddar_generated_liboko_h


#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



typedef void SpotPriceApi;

void* btc_spot_price_api_new(void);

void btc_spot_price_api_del(SpotPriceApi* handle);

typedef struct TickerData {
	double buy;
	double high;
	double last;
	double low;
	double sell;
	double vol;
	unsigned long long date;
} TickerData;

TickerData* btc_spot_price_api_get_ticker(SpotPriceApi* handle);



#ifdef __cplusplus
}
#endif


#endif
