static int f1(int *p, int *q) { *p += 826; return *q * 2; }
static int f2(int *p, int *q) { *p += 293; return *q * 2; }
static int f3(int *p, int *q) { *p += 317; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[1]);
    cells[0] = cells[6] + 524;
    cells[7] = cells[1] + 964;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 97u; }
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[3], &cells[1]);
    sum += f3(&cells[2], &cells[1]);
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    arr[0 % 6] = 398; cells[5] += arr[0 % 6];
    arr[3 % 6] = 384; cells[2] += arr[3 % 6];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 827u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }