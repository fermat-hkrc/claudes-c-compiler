static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[6 % 6] = 155; cells[7] += arr[6 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 993u; }
    static int f1(int *p, int *q) { *p += 842; return *q * 2; }
    sum += f1(&cells[4], &cells[7]);
    static int f2(int *p, int *q) { *p += 453; return *q * 2; }
    sum += f2(&cells[3], &cells[3]);
    static int f3(int *p, int *q) { *p += 903; return *q * 2; }
    sum += f3(&cells[3], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[5 % 6] = 949; cells[0] += arr[5 % 6];
    cells[5] = cells[3] + 551;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }