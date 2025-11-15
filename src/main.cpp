#include "stdio.h"

extern "C" {
void myfun() { printf("asd %d\n", 5 + 5); }
}
