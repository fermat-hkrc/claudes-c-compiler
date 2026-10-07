static int f1(int *p, int *q) { *p += 913; return *q * 2; }
static int f2(int *p, int *q) { *p += 44; return *q * 2; }
static int f3(int *p, int *q) { *p += 433; return *q * 2; }
static int f4(int *p, int *q) { *p += 27; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    cells[2] = cells[4] + 827;
    arr[4 % 6] = 403; cells[7] += arr[4 % 6];
    arr[3 % 6] = 605; cells[5] += arr[3 % 6];
    arr[1 % 6] = 533; cells[3] += arr[1 % 6];
    sum += f1(&cells[3], &cells[1]);
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += f2(&cells[6], &cells[0]);
    sum += f3(&cells[1], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    cells[7] = cells[5] + 185;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[3] = cells[4] + 477;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[3] = cells[0] + 677;
    cells[5] = cells[3] + 276;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    sum += f4(&cells[7], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }