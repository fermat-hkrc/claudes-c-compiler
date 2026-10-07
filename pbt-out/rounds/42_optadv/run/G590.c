static int f1(int *p, int *q) { *p += 578; return *q * 2; }
static int f2(int *p, int *q) { *p += 980; return *q * 2; }
static int f3(int *p, int *q) { *p += 334; return *q * 2; }
static int f4(int *p, int *q) { *p += 232; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    sum += f1(&cells[5], &cells[0]);
    sum += f2(&cells[3], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[6 % 6] = 167; cells[1] += arr[6 % 6];
    sum += f3(&cells[3], &cells[1]);
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    sum += f4(&cells[6], &cells[3]);
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 565u; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[3 % 6] = 315; cells[1] += arr[3 % 6];
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    arr[2 % 6] = 804; cells[7] += arr[2 % 6];
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 917u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }