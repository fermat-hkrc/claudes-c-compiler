static int f1(int *p, int *q) { *p += 360; return *q * 2; }
static int f2(int *p, int *q) { *p += 326; return *q * 2; }
static int f3(int *p, int *q) { *p += 281; return *q * 2; }
static int f4(int *p, int *q) { *p += 682; return *q * 2; }
static int f5(int *p, int *q) { *p += 685; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[6], &cells[1]);
    sum += f2(&cells[2], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 200u; }
    cells[0] = cells[2] + 1;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += f3(&cells[6], &cells[0]);
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 149u; }
    cells[5] = cells[1] + 225;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += f4(&cells[7], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[1 % 6] = 121; cells[3] += arr[1 % 6];
    sum += f5(&cells[4], &cells[3]);
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    arr[0 % 6] = 750; cells[7] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }