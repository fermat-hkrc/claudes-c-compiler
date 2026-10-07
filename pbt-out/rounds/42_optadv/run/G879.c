static int f1(int *p, int *q) { *p += 677; return *q * 2; }
static int f2(int *p, int *q) { *p += 715; return *q * 2; }
static int f3(int *p, int *q) { *p += 879; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[4] + 7;
    sum += f1(&cells[0], &cells[2]);
    arr[3 % 6] = 199; cells[0] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 327u; }
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[2], &cells[0]);
    sum += f3(&cells[6], &cells[6]);
    arr[1 % 6] = 364; cells[1] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }