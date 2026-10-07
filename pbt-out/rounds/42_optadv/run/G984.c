static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[0] + 913;
    cells[1] = cells[0] + 621;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    cells[1] = cells[0] + 567;
    arr[6 % 6] = 688; cells[3] += arr[6 % 6];
    arr[2 % 6] = 387; cells[4] += arr[2 % 6];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 617u; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 943u; }
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    static int f1(int *p, int *q) { *p += 483; return *q * 2; }
    sum += f1(&cells[1], &cells[0]);
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    cells[5] = cells[0] + 425;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    static int f2(int *p, int *q) { *p += 422; return *q * 2; }
    sum += f2(&cells[2], &cells[6]);
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 673u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f3(int *p, int *q) { *p += 203; return *q * 2; }
    sum += f3(&cells[4], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }