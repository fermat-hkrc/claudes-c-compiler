static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 333u; }
    static int f1(int *p, int *q) { *p += 990; return *q * 2; }
    sum += f1(&cells[3], &cells[2]);
    static int f2(int *p, int *q) { *p += 594; return *q * 2; }
    sum += f2(&cells[1], &cells[0]);
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 688u; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 497u; }
    cells[2] = cells[4] + 155;
    arr[7 % 6] = 239; cells[7] += arr[7 % 6];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 690u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 510u; }
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 191u; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    cells[6] = cells[5] + 49;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[7 % 6] = 142; cells[5] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }