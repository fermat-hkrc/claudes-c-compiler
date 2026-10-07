static int f1(int *p, int *q) { *p += 144; return *q * 2; }
static int f2(int *p, int *q) { *p += 790; return *q * 2; }
static int f3(int *p, int *q) { *p += 405; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[7] + 136;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 38; cells[3] += arr[2 % 6];
    sum += f1(&cells[1], &cells[4]);
    sum += f2(&cells[7], &cells[3]);
    sum += f3(&cells[0], &cells[7]);
    cells[6] = cells[6] + 68;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[1] = cells[0] + 248;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 544u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }