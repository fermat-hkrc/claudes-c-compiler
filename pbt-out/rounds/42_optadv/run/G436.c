static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f1(int *p, int *q) { *p += 296; return *q * 2; }
    sum += f1(&cells[4], &cells[0]);
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 179u; }
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 931u; }
    cells[3] = cells[1] + 272;
    cells[3] = cells[7] + 112;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 509u; }
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f2(int *p, int *q) { *p += 208; return *q * 2; }
    sum += f2(&cells[7], &cells[4]);
    static int f3(int *p, int *q) { *p += 67; return *q * 2; }
    sum += f3(&cells[1], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f4(int *p, int *q) { *p += 730; return *q * 2; }
    sum += f4(&cells[1], &cells[2]);
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 907u; }
    static int f5(int *p, int *q) { *p += 499; return *q * 2; }
    sum += f5(&cells[2], &cells[5]);
    arr[0 % 6] = 837; cells[0] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }