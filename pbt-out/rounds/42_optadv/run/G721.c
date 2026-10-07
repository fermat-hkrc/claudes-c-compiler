static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    arr[7 % 6] = 902; cells[7] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    cells[6] = cells[1] + 69;
    static int f1(int *p, int *q) { *p += 96; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    static int f2(int *p, int *q) { *p += 5; return *q * 2; }
    sum += f2(&cells[2], &cells[7]);
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 355u; }
    arr[5 % 6] = 647; cells[6] += arr[5 % 6];
    static int f3(int *p, int *q) { *p += 36; return *q * 2; }
    sum += f3(&cells[7], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[5 % 6] = 306; cells[7] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }