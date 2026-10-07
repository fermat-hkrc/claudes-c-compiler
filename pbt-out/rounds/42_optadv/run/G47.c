static int f1(int *p, int *q) { *p += 101; return *q * 2; }
static int f2(int *p, int *q) { *p += 733; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[1], &cells[1]);
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 719u; }
    sum += f2(&cells[0], &cells[1]);
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[4] = cells[7] + 928;
    arr[6 % 6] = 80; cells[5] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[5 % 6] = 909; cells[3] += arr[5 % 6];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    cells[1] = cells[4] + 358;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    cells[5] = cells[2] + 609;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }