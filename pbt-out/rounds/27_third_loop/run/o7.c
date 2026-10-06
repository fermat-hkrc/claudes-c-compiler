#include <stdio.h>
#include <string.h>
struct Vec3 { double x, y, z; };
static struct Vec3 cross(struct Vec3 a, struct Vec3 b) {
    struct Vec3 r = {
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    };
    return r;
}
static double dot(struct Vec3 a, struct Vec3 b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}
static struct Vec3 normalize(struct Vec3 v) {
    double d = dot(v, v);
    if (d == 0) return v;
    double inv = 1.0 / sqrt(d);
    return (struct Vec3){ v.x * inv, v.y * inv, v.z * inv };
}
int main(void) {
    struct Vec3 a = { 1, 0, 0 }, b = { 0, 1, 0 };
    struct Vec3 c = cross(a, b);
    struct Vec3 nc = normalize(c);
    struct Vec3 na = normalize(a);
    printf("%.3f %.3f %.3f\n", nc.x, nc.y, nc.z);
    printf("%.3f %.3f\n", na.x, dot(na, na));
    return 0;
}
