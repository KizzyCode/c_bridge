//
// Created by Keziah Elis Biermann on 08.11.19.
//

#include "c_bridge.h"


uint8_t dummy() {
	return 7;
}


uint64_t array_len(const array_u8_t* array) {
	return array->len(array);
}

void array_set0(array_u8_t* array) {
	uint8_t* data = array->data_mut(array);
	uint64_t len = array->len(array);
	for (uint64_t i = 0; i < len; i++) data[i] = (uint8_t)'0';
}