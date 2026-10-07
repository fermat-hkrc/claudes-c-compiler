static int f1(int *p, int *q) { *p += 24; return *q * 2; }
static int f2(int *p, int *q) { *p += 407; return *q * 2; }
static int f3(int *p, int *q) { *p += 241; return *q * 2; }
static int f4(int *p, int *q) { *p += 772; return *q * 2; }
static int f5(int *p, int *q) { *p += 435; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[2], &cells[1]);
    sum += f2(&cells[6], &cells[3]);
    sum += f3(&cells[4], &cells[0]);
    sum += f4(&cells[7], &cells[5]);
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 569u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 306u; }
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 832u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 334u; }
    sum += f5(&cells[1], &cells[4]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }