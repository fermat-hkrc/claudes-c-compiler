static int f1(int *p, int *q) { *p += 110; return *q * 2; }
static int f2(int *p, int *q) { *p += 457; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    sum += f1(&cells[2], &cells[4]);
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[2], &cells[4]);
    arr[6 % 6] = 920; cells[5] += arr[6 % 6];
    arr[7 % 6] = 730; cells[0] += arr[7 % 6];
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 157u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[3 % 6] = 228; cells[5] += arr[3 % 6];
    cells[6] = cells[1] + 880;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 375u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }