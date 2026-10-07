static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 881u; }
    arr[5 % 6] = 56; cells[3] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    cells[2] = cells[2] + 789;
    arr[3 % 6] = 514; cells[0] += arr[3 % 6];
    arr[6 % 6] = 459; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[7 % 6] = 44; cells[4] += arr[7 % 6];
    static int f1(int *p, int *q) { *p += 976; return *q * 2; }
    sum += f1(&cells[7], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    static int f2(int *p, int *q) { *p += 466; return *q * 2; }
    sum += f2(&cells[7], &cells[5]);
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 154u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 580u; }
    cells[7] = cells[0] + 650;
    arr[4 % 6] = 249; cells[7] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 570u; }
    arr[5 % 6] = 494; cells[1] += arr[5 % 6];
    static int f3(int *p, int *q) { *p += 934; return *q * 2; }
    sum += f3(&cells[4], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }