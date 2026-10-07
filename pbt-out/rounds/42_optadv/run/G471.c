static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 849; return *q * 2; }
    sum += f1(&cells[3], &cells[6]);
    arr[6 % 6] = 885; cells[0] += arr[6 % 6];
    static int f2(int *p, int *q) { *p += 147; return *q * 2; }
    sum += f2(&cells[4], &cells[7]);
    cells[0] = cells[1] + 343;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    arr[1 % 6] = 415; cells[7] += arr[1 % 6];
    arr[5 % 6] = 143; cells[1] += arr[5 % 6];
    static int f3(int *p, int *q) { *p += 353; return *q * 2; }
    sum += f3(&cells[6], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    cells[3] = cells[7] + 681;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 217u; }
    cells[5] = cells[0] + 750;
    static int f4(int *p, int *q) { *p += 532; return *q * 2; }
    sum += f4(&cells[6], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }