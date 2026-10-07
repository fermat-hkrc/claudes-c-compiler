static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    arr[2 % 6] = 145; cells[1] += arr[2 % 6];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    arr[7 % 6] = 221; cells[1] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    static int f1(int *p, int *q) { *p += 43; return *q * 2; }
    sum += f1(&cells[6], &cells[3]);
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 853u; }
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 589u; }
    static int f2(int *p, int *q) { *p += 746; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    cells[1] = cells[7] + 166;
    arr[4 % 6] = 498; cells[2] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }