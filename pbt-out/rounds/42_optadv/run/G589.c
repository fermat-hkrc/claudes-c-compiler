static int f1(int *p, int *q) { *p += 692; return *q * 2; }
static int f2(int *p, int *q) { *p += 619; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[1], &cells[5]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 966u; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    cells[5] = cells[1] + 220;
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 473u; }
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    arr[7 % 6] = 747; cells[7] += arr[7 % 6];
    arr[6 % 6] = 740; cells[0] += arr[6 % 6];
    arr[0 % 6] = 100; cells[6] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[6], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }