static int f1(int *p, int *q) { *p += 225; return *q * 2; }
static int f2(int *p, int *q) { *p += 526; return *q * 2; }
static int f3(int *p, int *q) { *p += 283; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[7] = cells[2] + 323;
    sum += f1(&cells[7], &cells[7]);
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 630u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 181u; }
    sum += f2(&cells[2], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    cells[1] = cells[2] + 965;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[0] = cells[1] + 882;
    sum += f3(&cells[3], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }