static int f1(int *p, int *q) { *p += 24; return *q * 2; }
static int f2(int *p, int *q) { *p += 265; return *q * 2; }
static int f3(int *p, int *q) { *p += 626; return *q * 2; }
static int f4(int *p, int *q) { *p += 90; return *q * 2; }
static int f5(int *p, int *q) { *p += 992; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 141; cells[0] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[0], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 815u; }
    cells[4] = cells[6] + 193;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[7] = cells[0] + 240;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    arr[7 % 6] = 246; cells[2] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 668u; }
    sum += f2(&cells[6], &cells[6]);
    sum += f3(&cells[2], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    cells[2] = cells[1] + 341;
    sum += f4(&cells[2], &cells[4]);
    sum += f5(&cells[2], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }