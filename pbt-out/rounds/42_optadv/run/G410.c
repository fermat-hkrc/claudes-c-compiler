static int f1(int *p, int *q) { *p += 783; return *q * 2; }
static int f2(int *p, int *q) { *p += 135; return *q * 2; }
static int f3(int *p, int *q) { *p += 597; return *q * 2; }
static int f4(int *p, int *q) { *p += 375; return *q * 2; }
static int f5(int *p, int *q) { *p += 458; return *q * 2; }
static int f6(int *p, int *q) { *p += 242; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f1(&cells[4], &cells[1]);
    sum += f2(&cells[3], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 551u; }
    cells[5] = cells[3] + 370;
    sum += f3(&cells[0], &cells[7]);
    sum += f4(&cells[5], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[5] = cells[3] + 853;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    cells[4] = cells[3] + 633;
    arr[3 % 6] = 940; cells[6] += arr[3 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += f5(&cells[6], &cells[0]);
    sum += f6(&cells[6], &cells[7]);
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 342u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }