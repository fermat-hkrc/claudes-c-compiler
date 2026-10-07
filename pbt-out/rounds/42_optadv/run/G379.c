static int f1(int *p, int *q) { *p += 234; return *q * 2; }
static int f2(int *p, int *q) { *p += 78; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 511; cells[2] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    cells[5] = cells[3] + 117;
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    sum += f1(&cells[0], &cells[5]);
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 683u; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 962u; }
    sum += f2(&cells[7], &cells[1]);
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[5 % 6] = 184; cells[6] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }