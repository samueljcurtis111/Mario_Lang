#include "stdio.h"
int main()
{
    char tape[20000] = {0};
    char *ptr = tape;
	*ptr += 10;
	while (*ptr) {
	++ptr;
	*ptr += 7;
	++ptr;
	*ptr += 10;
	++ptr;
	*ptr += 11;
	++ptr;
	*ptr += 11;
	--ptr;
	--ptr;
	--ptr;
	--ptr;
	--*ptr;
	}
	++ptr;
	*ptr += 7;
	putchar(*ptr);
	++ptr;
	*ptr -= 15;
	putchar(*ptr);
	++ptr;
	*ptr += 4;
	putchar(*ptr);
	++ptr;
	*ptr -= 13;
	putchar(*ptr);
	*ptr += 6;
	putchar(*ptr);
}
