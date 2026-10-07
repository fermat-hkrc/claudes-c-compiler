static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 950; cells[1] += arr[5 % 6];
    cells[4] = cells[7] + 343;
    arr[7 % 6] = 875; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    cells[7] = cells[3] + 733;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 263u; }
    arr[6 % 6] = 730; cells[3] += arr[6 % 6];
    cells[4] = cells[4] + 833;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[5] = cells[1] + 173;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 123u; }
    static int f1(int *p, int *q) { *p += 201; return *q * 2; }
    sum += f1(&cells[0], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    cells[3] = cells[7] + 714;
    arr[4 % 6] = 984; cells[2] += arr[4 % 6];
    arr[1 % 6] = 65; cells[5] += arr[1 % 6];
    arr[0 % 6] = 705; cells[6] += arr[0 % 6];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 297u; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 167u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }