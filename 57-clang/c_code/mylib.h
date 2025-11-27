// c_code/mylib.h
#ifndef MYLIB_H
#define MYLIB_H

#ifdef __cplusplus
extern "C" {
#endif

// Simple add function
int add_ints(int a, int b);

// Greet function (just prints using printf)
void greet(const char *name);

#ifdef __cplusplus
}
#endif

#endif // MYLIB_H