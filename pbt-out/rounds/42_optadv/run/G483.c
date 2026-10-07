static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[1] + 806;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    cells[5] = cells[1] + 551;
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 712u; }
    static int f1(int *p, int *q) { *p += 346; return *q * 2; }
    sum += f1(&cells[4], &cells[6]);
    static int f2(int *p, int *q) { *p += 670; return *q * 2; }
    sum += f2(&cells[1], &cells[7]);
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 354u; }
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    cells[3] = cells[0] + 306;
    static int f3(int *p, int *q) { *p += 173; return *q * 2; }
    sum += f3(&cells[4], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }