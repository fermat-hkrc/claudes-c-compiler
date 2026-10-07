static int f1(int *p, int *q) { *p += 298; return *q * 2; }
static int f2(int *p, int *q) { *p += 142; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 605; cells[1] += arr[4 % 6];
    arr[6 % 6] = 150; cells[7] += arr[6 % 6];
    sum += f1(&cells[0], &cells[7]);
    arr[5 % 6] = 673; cells[2] += arr[5 % 6];
    cells[5] = cells[7] + 71;
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 789u; }
    arr[7 % 6] = 754; cells[6] += arr[7 % 6];
    arr[1 % 6] = 644; cells[4] += arr[1 % 6];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f2(&cells[0], &cells[1]);
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    arr[0 % 6] = 674; cells[2] += arr[0 % 6];
    arr[4 % 6] = 86; cells[6] += arr[4 % 6];
    arr[4 % 6] = 974; cells[3] += arr[4 % 6];
    cells[1] = cells[7] + 223;
    cells[5] = cells[6] + 299;
    cells[0] = cells[5] + 504;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }