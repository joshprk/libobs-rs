#include <math.h>
#include <stdio.h> // Include the standard I/O header for printf
#include "obs/graphics/vec4.h"

struct vec4 vec4_create() {
    struct vec4 v;

    return v;
}

void vec4_set1(struct vec4 *v, float x, float y, float z, float w) {
//printf("Parameters: x = %f, y = %f, z = %f, w = %f\n", x, y, z, w);
    vec4_set(v, x, y, z, w);

//printf("vec4_set1: x = %f, y = %f, z = %f, w = %f m=%f\n", v->x, v->y, v->z, v->w, _mm_set_ps(w, z, y, x));
}