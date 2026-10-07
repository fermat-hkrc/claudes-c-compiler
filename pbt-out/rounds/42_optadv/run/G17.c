static int f1(int *p, int *q) { *p += 19; return *q * 2; }
static int f2(int *p, int *q) { *p += 777; return *q * 2; }
static int f3(int *p, int *q) { *p += 332; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[1] + 943;
    sum += f1(&cells[6], &cells[3]);
    arr[4 % 6] = 944; cells[0] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 610u; }
    arr[5 % 6] = 951; cells[6] += arr[5 % 6];
    cells[3] = cells[3] + 475;
    arr[2 % 6] = 539; cells[3] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 831u; }
    arr[6 % 6] = 513; cells[7] += arr[6 % 6];
    sum += f2(&cells[3], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[4 % 6] = 266; cells[3] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    sum += f3(&cells[3], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }