static int f1(int *p, int *q) { *p += 717; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[7] + 537;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 181u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[5] = cells[0] + 620;
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    cells[2] = cells[1] + 764;
    cells[3] = cells[6] + 739;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f1(&cells[4], &cells[4]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    cells[0] = cells[6] + 581;
    cells[2] = cells[3] + 724;
    cells[7] = cells[2] + 332;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }