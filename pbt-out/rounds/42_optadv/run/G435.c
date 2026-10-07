static int f1(int *p, int *q) { *p += 874; return *q * 2; }
static int f2(int *p, int *q) { *p += 325; return *q * 2; }
static int f3(int *p, int *q) { *p += 598; return *q * 2; }
static int f4(int *p, int *q) { *p += 679; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[0], &cells[0]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    sum += f2(&cells[4], &cells[4]);
    arr[7 % 6] = 598; cells[7] += arr[7 % 6];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f3(&cells[6], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 120u; }
    sum += f4(&cells[5], &cells[5]);
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 938u; }
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }