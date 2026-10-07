static int f1(int *p, int *q) { *p += 707; return *q * 2; }
static int f2(int *p, int *q) { *p += 475; return *q * 2; }
static int f3(int *p, int *q) { *p += 632; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 473; cells[3] += arr[6 % 6];
    sum += f1(&cells[1], &cells[2]);
    arr[0 % 6] = 342; cells[6] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[6 % 6] = 334; cells[5] += arr[6 % 6];
    sum += f2(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f3(&cells[4], &cells[6]);
    arr[5 % 6] = 325; cells[1] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 472u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }