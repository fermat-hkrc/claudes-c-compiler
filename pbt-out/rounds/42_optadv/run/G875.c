static int f1(int *p, int *q) { *p += 510; return *q * 2; }
static int f2(int *p, int *q) { *p += 560; return *q * 2; }
static int f3(int *p, int *q) { *p += 30; return *q * 2; }
static int f4(int *p, int *q) { *p += 657; return *q * 2; }
static int f5(int *p, int *q) { *p += 633; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 353; cells[0] += arr[5 % 6];
    arr[5 % 6] = 984; cells[3] += arr[5 % 6];
    cells[4] = cells[2] + 213;
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[7], &cells[0]);
    arr[2 % 6] = 473; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += f2(&cells[0], &cells[5]);
    arr[0 % 6] = 873; cells[3] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    arr[5 % 6] = 542; cells[0] += arr[5 % 6];
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f3(&cells[6], &cells[3]);
    sum += f4(&cells[2], &cells[4]);
    sum += f5(&cells[4], &cells[1]);
    cells[1] = cells[4] + 786;
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }