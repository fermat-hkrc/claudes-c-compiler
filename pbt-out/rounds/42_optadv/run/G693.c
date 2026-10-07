static int f1(int *p, int *q) { *p += 913; return *q * 2; }
static int f2(int *p, int *q) { *p += 117; return *q * 2; }
static int f3(int *p, int *q) { *p += 133; return *q * 2; }
static int f4(int *p, int *q) { *p += 972; return *q * 2; }
static int f5(int *p, int *q) { *p += 176; return *q * 2; }
static int f6(int *p, int *q) { *p += 389; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 429u; }
    sum += f1(&cells[0], &cells[7]);
    sum += f2(&cells[1], &cells[4]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    sum += f3(&cells[2], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 178u; }
    sum += f4(&cells[5], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    cells[7] = cells[1] + 645;
    arr[3 % 6] = 548; cells[0] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += f5(&cells[5], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[1 % 6] = 98; cells[5] += arr[1 % 6];
    cells[5] = cells[0] + 842;
    sum += f6(&cells[1], &cells[6]);
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }