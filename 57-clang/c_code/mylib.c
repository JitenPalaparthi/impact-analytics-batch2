// c_code/mylib.c
#include <stdio.h>
#include "mylib.h"

int add_ints(int a, int b) {
    return a + b;
}

void greet(const char *name) {
    printf("Hello %s from C library!\\n", name);
}