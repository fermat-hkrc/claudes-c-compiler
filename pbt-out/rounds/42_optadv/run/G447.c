static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[1] = cells[7] + 875;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    static int f1(int *p, int *q) { *p += 993; return *q * 2; }
    sum += f1(&cells[6], &cells[2]);
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    arr[7 % 6] = 92; cells[7] += arr[7 % 6];
    cells[7] = cells[3] + 498;
    arr[2 % 6] = 907; cells[6] += arr[2 % 6];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 835u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }