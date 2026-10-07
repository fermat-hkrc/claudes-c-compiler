static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 5u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 207u; }
    cells[2] = cells[1] + 20;
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    arr[3 % 6] = 335; cells[4] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 847; return *q * 2; }
    sum += f1(&cells[0], &cells[2]);
    arr[3 % 6] = 137; cells[0] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 28u; }
    static int f2(int *p, int *q) { *p += 414; return *q * 2; }
    sum += f2(&cells[2], &cells[5]);
    arr[4 % 6] = 581; cells[4] += arr[4 % 6];
    arr[0 % 6] = 496; cells[7] += arr[0 % 6];
    cells[6] = cells[4] + 379;
    static int f3(int *p, int *q) { *p += 190; return *q * 2; }
    sum += f3(&cells[6], &cells[0]);
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 809u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }