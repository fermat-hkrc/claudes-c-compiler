static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[5] = cells[2] + 550;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    static int f1(int *p, int *q) { *p += 247; return *q * 2; }
    sum += f1(&cells[2], &cells[7]);
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    arr[1 % 6] = 14; cells[2] += arr[1 % 6];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 774u; }
    arr[3 % 6] = 252; cells[1] += arr[3 % 6];
    arr[7 % 6] = 692; cells[5] += arr[7 % 6];
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 183u; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 189u; }
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    static int f2(int *p, int *q) { *p += 231; return *q * 2; }
    sum += f2(&cells[3], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[5] = cells[1] + 872;
    cells[0] = cells[7] + 320;
    arr[5 % 6] = 355; cells[6] += arr[5 % 6];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }