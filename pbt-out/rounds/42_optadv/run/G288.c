static int f1(int *p, int *q) { *p += 571; return *q * 2; }
static int f2(int *p, int *q) { *p += 523; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 208u; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    arr[5 % 6] = 401; cells[5] += arr[5 % 6];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[6], &cells[3]);
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 442u; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 471u; }
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += f2(&cells[5], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }