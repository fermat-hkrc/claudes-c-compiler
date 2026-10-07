static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[3 % 6] = 431; cells[0] += arr[3 % 6];
    cells[2] = cells[6] + 21;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 464u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 168u; }
    cells[0] = cells[5] + 172;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 601u; }
    cells[3] = cells[4] + 462;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 706u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f1(int *p, int *q) { *p += 806; return *q * 2; }
    sum += f1(&cells[0], &cells[3]);
    cells[5] = cells[6] + 864;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    cells[3] = cells[2] + 793;
    arr[6 % 6] = 189; cells[3] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }