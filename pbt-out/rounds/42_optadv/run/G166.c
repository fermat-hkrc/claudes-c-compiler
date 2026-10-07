static int f1(int *p, int *q) { *p += 991; return *q * 2; }
static int f2(int *p, int *q) { *p += 620; return *q * 2; }
static int f3(int *p, int *q) { *p += 880; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 459u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 517u; }
    arr[2 % 6] = 267; cells[1] += arr[2 % 6];
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 458u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[3], &cells[3]);
    sum += f2(&cells[5], &cells[7]);
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    cells[0] = cells[0] + 54;
    sum += f3(&cells[7], &cells[5]);
    cells[7] = cells[5] + 72;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 257u; }
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }