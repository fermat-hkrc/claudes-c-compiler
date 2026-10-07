static int f1(int *p, int *q) { *p += 373; return *q * 2; }
static int f2(int *p, int *q) { *p += 268; return *q * 2; }
static int f3(int *p, int *q) { *p += 640; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 638; cells[7] += arr[5 % 6];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 588u; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    arr[2 % 6] = 496; cells[2] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    arr[7 % 6] = 68; cells[4] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    arr[7 % 6] = 673; cells[2] += arr[7 % 6];
    sum += f1(&cells[1], &cells[1]);
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 965u; }
    sum += f2(&cells[2], &cells[0]);
    sum += f3(&cells[2], &cells[5]);
    cells[7] = cells[0] + 116;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }