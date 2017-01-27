#include <stdio.h>
#include "liboko.h"

int main(int argc, char* argv[]) {
	void* a = btc_spot_price_api_new();
	printf("p: %xd \n",a);

	btc_spot_price_api_get_ticker(a);

	btc_spot_price_api_del(a);
	printf("p: %xd \n",a);

	return 0;
}
