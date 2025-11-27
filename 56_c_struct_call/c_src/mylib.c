// c_src/mylib.c
#include <stdio.h>
#include "mylib.h"

// ----- Point functions -----

Point make_point(int x, int y) {
    Point p;
    p.x = x;
    p.y = y;
    return p;
}

int manhattan_distance(const Point *p) {
    if (!p) {
        return -1;
    }
    int ax = p->x >= 0 ? p->x : -p->x;
    int ay = p->y >= 0 ? p->y : -p->y;
    return ax + ay;
}

// ----- Person functions -----

Person make_person(const char *name, int age) {
    Person p;
    p.name = name;
    p.age  = age;
    return p;
}

void print_person(const Person *p) {
    if (!p) {
        printf("Person is NULL\n");
        return;
    }
    printf("Person{name=\"%s\", age=%d}\n", p->name, p->age);
}

int birthday(Person *p) {
    if (!p) {
        return -1;
    }
    p->age += 1;
    return p->age;
}