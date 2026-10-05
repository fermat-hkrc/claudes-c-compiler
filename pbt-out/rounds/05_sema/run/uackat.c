int main(void) {
    unsigned long a; long long b;
    _Static_assert(__builtin_types_compatible_p(typeof(a + b), unsigned long long), "ul+ll -> ull");
    unsigned int c; long d;
    _Static_assert(__builtin_types_compatible_p(typeof(c + d), long), "ui+l -> l");
    unsigned long e; int f;
    _Static_assert(__builtin_types_compatible_p(typeof(e + f), unsigned long), "ul+i -> ul");
    return 0;
}
