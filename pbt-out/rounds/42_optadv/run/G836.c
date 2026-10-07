static int f1(int *p, int *q) { *p += 380; return *q * 2; }
static int f2(int *p, int *q) { *p += 316; return *q * 2; }
static int f3(int *p, int *q) { *p += 520; return *q * 2; }
static int f4(int *p, int *q) { *p += 628; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[2] = cells[1] + 604;
    sum += f2(&cells[5], &cells[3]);
    arr[0 % 6] = 698; cells[5] += arr[0 % 6];
    cells[1] = cells[6] + 698;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    arr[6 % 6] = 448; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[0] = cells[5] + 763;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += f3(&cells[4], &cells[2]);
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    cells[3] = cells[0] + 418;
    sum += f4(&cells[4], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }