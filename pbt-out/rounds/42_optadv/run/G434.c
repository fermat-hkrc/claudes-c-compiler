static int f1(int *p, int *q) { *p += 797; return *q * 2; }
static int f2(int *p, int *q) { *p += 59; return *q * 2; }
static int f3(int *p, int *q) { *p += 52; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[4] + 383;
    sum += f1(&cells[6], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += f2(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    cells[7] = cells[0] + 390;
    arr[5 % 6] = 531; cells[1] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += f3(&cells[3], &cells[2]);
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    arr[3 % 6] = 566; cells[7] += arr[3 % 6];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    cells[4] = cells[7] + 867;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }