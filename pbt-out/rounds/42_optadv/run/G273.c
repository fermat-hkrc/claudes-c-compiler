static int f1(int *p, int *q) { *p += 819; return *q * 2; }
static int f2(int *p, int *q) { *p += 156; return *q * 2; }
static int f3(int *p, int *q) { *p += 885; return *q * 2; }
static int f4(int *p, int *q) { *p += 392; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 427; cells[2] += arr[7 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    sum += f1(&cells[1], &cells[0]);
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += f2(&cells[5], &cells[7]);
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 978u; }
    sum += f3(&cells[3], &cells[2]);
    arr[3 % 6] = 821; cells[3] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 242u; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 557u; }
    sum += f4(&cells[4], &cells[2]);
    cells[5] = cells[5] + 341;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }