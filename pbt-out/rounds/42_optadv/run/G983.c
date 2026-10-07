static int f1(int *p, int *q) { *p += 825; return *q * 2; }
static int f2(int *p, int *q) { *p += 651; return *q * 2; }
static int f3(int *p, int *q) { *p += 421; return *q * 2; }
static int f4(int *p, int *q) { *p += 870; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f1(&cells[1], &cells[3]);
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    sum += f2(&cells[7], &cells[6]);
    sum += f3(&cells[0], &cells[0]);
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 803u; }
    cells[6] = cells[4] + 976;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 439u; }
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    sum += f4(&cells[7], &cells[2]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }