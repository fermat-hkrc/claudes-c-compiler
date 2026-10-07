static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    arr[6 % 6] = 580; cells[2] += arr[6 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 728u; }
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 205u; }
    arr[1 % 6] = 330; cells[3] += arr[1 % 6];
    static int f1(int *p, int *q) { *p += 543; return *q * 2; }
    sum += f1(&cells[3], &cells[1]);
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    arr[6 % 6] = 987; cells[4] += arr[6 % 6];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 65u; }
    cells[0] = cells[4] + 698;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    static int f2(int *p, int *q) { *p += 353; return *q * 2; }
    sum += f2(&cells[6], &cells[5]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f3(int *p, int *q) { *p += 712; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f4(int *p, int *q) { *p += 605; return *q * 2; }
    sum += f4(&cells[1], &cells[5]);
    arr[0 % 6] = 687; cells[7] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }