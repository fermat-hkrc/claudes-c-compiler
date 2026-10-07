static int f1(int *p, int *q) { *p += 499; return *q * 2; }
static int f2(int *p, int *q) { *p += 825; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[4]);
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 414u; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 323u; }
    cells[1] = cells[5] + 942;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    arr[2 % 6] = 964; cells[0] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    arr[0 % 6] = 15; cells[4] += arr[0 % 6];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 381u; }
    sum += f2(&cells[1], &cells[7]);
    cells[7] = cells[7] + 258;
    arr[6 % 6] = 152; cells[2] += arr[6 % 6];
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 630u; }
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    arr[5 % 6] = 310; cells[6] += arr[5 % 6];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }