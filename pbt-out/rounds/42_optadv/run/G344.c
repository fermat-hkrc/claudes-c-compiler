static int f1(int *p, int *q) { *p += 370; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    cells[1] = cells[4] + 576;
    cells[0] = cells[0] + 366;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 186u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 196; cells[6] += arr[1 % 6];
    cells[5] = cells[5] + 189;
    cells[5] = cells[1] + 414;
    arr[5 % 6] = 533; cells[6] += arr[5 % 6];
    cells[3] = cells[0] + 625;
    cells[7] = cells[3] + 613;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    sum += f1(&cells[4], &cells[3]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    arr[3 % 6] = 372; cells[5] += arr[3 % 6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    cells[7] = cells[6] + 733;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }