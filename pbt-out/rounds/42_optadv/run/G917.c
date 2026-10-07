static int f1(int *p, int *q) { *p += 53; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 170u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[5] = cells[5] + 0;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 177u; }
    cells[2] = cells[3] + 799;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 713u; }
    sum += f1(&cells[6], &cells[6]);
    arr[6 % 6] = 214; cells[5] += arr[6 % 6];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    arr[7 % 6] = 405; cells[3] += arr[7 % 6];
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 85u; }
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }