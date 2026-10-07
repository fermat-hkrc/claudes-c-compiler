static int f1(int *p, int *q) { *p += 409; return *q * 2; }
static int f2(int *p, int *q) { *p += 649; return *q * 2; }
static int f3(int *p, int *q) { *p += 107; return *q * 2; }
static int f4(int *p, int *q) { *p += 732; return *q * 2; }
static int f5(int *p, int *q) { *p += 247; return *q * 2; }
static int f6(int *p, int *q) { *p += 700; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    sum += f1(&cells[0], &cells[6]);
    sum += f2(&cells[7], &cells[1]);
    arr[7 % 6] = 886; cells[2] += arr[7 % 6];
    arr[5 % 6] = 137; cells[0] += arr[5 % 6];
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    sum += f3(&cells[4], &cells[4]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += f4(&cells[6], &cells[3]);
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 600u; }
    sum += f5(&cells[0], &cells[3]);
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    sum += f6(&cells[7], &cells[1]);
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 651u; }
    cells[3] = cells[1] + 241;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }