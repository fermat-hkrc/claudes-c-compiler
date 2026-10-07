static int f1(int *p, int *q) { *p += 987; return *q * 2; }
static int f2(int *p, int *q) { *p += 180; return *q * 2; }
static int f3(int *p, int *q) { *p += 291; return *q * 2; }
static int f4(int *p, int *q) { *p += 74; return *q * 2; }
static int f5(int *p, int *q) { *p += 78; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[6] = cells[4] + 669;
    sum += f1(&cells[5], &cells[0]);
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[3] = cells[3] + 292;
    cells[7] = cells[7] + 561;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[7] = cells[2] + 851;
    cells[4] = cells[3] + 908;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    cells[0] = cells[3] + 424;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f2(&cells[2], &cells[5]);
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += f3(&cells[6], &cells[7]);
    sum += f4(&cells[6], &cells[2]);
    sum += f5(&cells[7], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }