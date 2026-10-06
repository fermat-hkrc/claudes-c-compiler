struct Big { long a, b, c; };
struct Big scale(struct Big v, long k) { struct Big r = { v.a*k, v.b*k, v.c*k }; return r; }
int main(void) { struct Big v = scale((struct Big){ 1, 2, 3 }, 7); printf("%ld %ld %ld\n", v.a, v.b, v.c); return 0; }
