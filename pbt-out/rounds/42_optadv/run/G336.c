static int f1(int *p, int *q) { *p += 483; return *q * 2; }
static int f2(int *p, int *q) { *p += 89; return *q * 2; }
static int f3(int *p, int *q) { *p += 10; return *q * 2; }
static int f4(int *p, int *q) { *p += 611; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[2 % 6] = 711; cells[7] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[7 % 6] = 857; cells[4] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    cells[5] = cells[4] + 815;
    arr[0 % 6] = 944; cells[5] += arr[0 % 6];
    sum += f1(&cells[7], &cells[2]);
    arr[4 % 6] = 112; cells[0] += arr[4 % 6];
    sum += f2(&cells[1], &cells[6]);
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 265u; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += f3(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    arr[3 % 6] = 321; cells[2] += arr[3 % 6];
    sum += f4(&cells[5], &cells[6]);
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 257u; }
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 414u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }