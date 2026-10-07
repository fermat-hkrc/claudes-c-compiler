static int f1(int *p, int *q) { *p += 320; return *q * 2; }
static int f2(int *p, int *q) { *p += 36; return *q * 2; }
static int f3(int *p, int *q) { *p += 750; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[5], &cells[2]);
    cells[3] = cells[6] + 541;
    sum += f2(&cells[4], &cells[7]);
    arr[6 % 6] = 14; cells[4] += arr[6 % 6];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 894u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f3(&cells[3], &cells[1]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 684u; }
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }