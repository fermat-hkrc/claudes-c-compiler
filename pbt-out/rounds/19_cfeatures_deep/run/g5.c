__int128 big(void) { return (__int128)1 << 100; }
unsigned long long lo(__int128 v) { return (unsigned long long)v; }
long long hi(__int128 v) { return (long long)(v >> 64); }
int main(void) { __int128 v = big(); printf("%lld %llu %lld\n", hi(v), lo(v), hi(v * 3)); return 0; }
