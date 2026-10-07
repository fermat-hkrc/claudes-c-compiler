static int f1(int *p, int *q) { *p += 150; return *q * 2; }
static int f2(int *p, int *q) { *p += 886; return *q * 2; }
static int f3(int *p, int *q) { *p += 200; return *q * 2; }
static int f4(int *p, int *q) { *p += 877; return *q * 2; }
static int f5(int *p, int *q) { *p += 568; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[4]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[2 % 6] = 824; cells[3] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[6 % 6] = 733; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f2(&cells[3], &cells[4]);
    sum += f3(&cells[2], &cells[1]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 605u; }
    sum += f4(&cells[6], &cells[4]);
    cells[7] = cells[2] + 236;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f5(&cells[2], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }