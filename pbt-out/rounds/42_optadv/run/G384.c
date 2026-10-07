static int f1(int *p, int *q) { *p += 487; return *q * 2; }
static int f2(int *p, int *q) { *p += 659; return *q * 2; }
static int f3(int *p, int *q) { *p += 462; return *q * 2; }
static int f4(int *p, int *q) { *p += 214; return *q * 2; }
static int f5(int *p, int *q) { *p += 18; return *q * 2; }
static int f6(int *p, int *q) { *p += 329; return *q * 2; }
static int f7(int *p, int *q) { *p += 244; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 493; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[5], &cells[7]);
    arr[0 % 6] = 155; cells[7] += arr[0 % 6];
    sum += f2(&cells[6], &cells[4]);
    sum += f3(&cells[1], &cells[6]);
    cells[5] = cells[5] + 165;
    arr[3 % 6] = 424; cells[0] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    cells[6] = cells[3] + 381;
    sum += f4(&cells[5], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f5(&cells[0], &cells[7]);
    sum += f6(&cells[7], &cells[1]);
    cells[5] = cells[3] + 40;
    sum += f7(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }