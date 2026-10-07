static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 705; cells[7] += arr[1 % 6];
    arr[3 % 6] = 596; cells[4] += arr[3 % 6];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 310u; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 494u; }
    static int f1(int *p, int *q) { *p += 374; return *q * 2; }
    sum += f1(&cells[4], &cells[2]);
    cells[7] = cells[3] + 776;
    static int f2(int *p, int *q) { *p += 970; return *q * 2; }
    sum += f2(&cells[0], &cells[1]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 814u; }
    static int f3(int *p, int *q) { *p += 594; return *q * 2; }
    sum += f3(&cells[5], &cells[0]);
    arr[7 % 6] = 749; cells[1] += arr[7 % 6];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 821u; }
    arr[5 % 6] = 770; cells[1] += arr[5 % 6];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 250u; }
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    static int f4(int *p, int *q) { *p += 151; return *q * 2; }
    sum += f4(&cells[2], &cells[1]);
    cells[3] = cells[5] + 382;
    arr[0 % 6] = 561; cells[0] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }