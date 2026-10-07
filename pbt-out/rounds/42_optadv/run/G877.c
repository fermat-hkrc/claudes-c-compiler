static int f1(int *p, int *q) { *p += 662; return *q * 2; }
static int f2(int *p, int *q) { *p += 939; return *q * 2; }
static int f3(int *p, int *q) { *p += 924; return *q * 2; }
static int f4(int *p, int *q) { *p += 63; return *q * 2; }
static int f5(int *p, int *q) { *p += 836; return *q * 2; }
static int f6(int *p, int *q) { *p += 133; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[5]);
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 604u; }
    sum += f2(&cells[0], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f3(&cells[5], &cells[4]);
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    sum += f4(&cells[6], &cells[5]);
    cells[7] = cells[6] + 637;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 305u; }
    sum += f5(&cells[5], &cells[2]);
    cells[6] = cells[3] + 238;
    cells[4] = cells[4] + 378;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 106u; }
    sum += f6(&cells[6], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }