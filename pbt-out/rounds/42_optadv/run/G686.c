static int f1(int *p, int *q) { *p += 311; return *q * 2; }
static int f2(int *p, int *q) { *p += 426; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f2(&cells[5], &cells[0]);
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    cells[1] = cells[0] + 828;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 826u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    cells[0] = cells[3] + 462;
    arr[4 % 6] = 9; cells[0] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }