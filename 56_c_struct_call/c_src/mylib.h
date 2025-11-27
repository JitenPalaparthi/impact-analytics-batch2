#ifndef MYLIB_H
#define MYLIB_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int x;
    int y;
} Point;

typedef struct {
    const char *name;
    int age;
} Person;

Point make_point(int x, int y);
int manhattan_distance(const Point *p);

Person make_person(const char *name, int age);
void print_person(const Person *p);
int birthday(Person *p);

#ifdef __cplusplus
}
#endif

#endif // MYLIB_H