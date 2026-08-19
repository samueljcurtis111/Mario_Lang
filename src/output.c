#include "stdio.h"
int main()
{
    char tape[20000] = {0};
    char *ptr = tape;
	*ptr = 0;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 8;
	while (*ptr) {
	++ptr;
	*ptr = 0;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 8;
	while (*ptr) {
	++ptr;
	*ptr = 0;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 16;
	*ptr += 8;
	while (*ptr) {
	++ptr;
	++*ptr;
	--ptr;
	--*ptr;
	}
	--ptr;
	--*ptr;
	}
	--ptr;
	--*ptr;
	}
	++ptr;
	++ptr;
	++ptr;
	putchar(*ptr);
}
