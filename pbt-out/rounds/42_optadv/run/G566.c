static int f1(int *p, int *q) { *p += 185; return *q * 2; }
static int f2(int *p, int *q) { *p += 721; return *q * 2; }
static int f3(int *p, int *q) { *p += 784; return *q * 2; }
static int f4(int *p, int *q) { *p += 698; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[4] + 216;
    sum += f1(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += f2(&cells[7], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    cells[1] = cells[6] + 520;
    sum += f3(&cells[2], &cells[4]);
    sum += f4(&cells[6], &cells[1]);
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 7u; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 349u; }
    arr[2 % 6] = 917; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }