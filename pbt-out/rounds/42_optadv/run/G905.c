static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[4 % 6] = 435; cells[7] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    cells[3] = cells[1] + 435;
    arr[7 % 6] = 771; cells[5] += arr[7 % 6];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    cells[7] = cells[3] + 641;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 339u; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 228u; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 337u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 180u; }
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 870u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }