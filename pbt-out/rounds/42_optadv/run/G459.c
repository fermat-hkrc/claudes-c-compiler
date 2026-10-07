static int f1(int *p, int *q) { *p += 111; return *q * 2; }
static int f2(int *p, int *q) { *p += 165; return *q * 2; }
static int f3(int *p, int *q) { *p += 909; return *q * 2; }
static int f4(int *p, int *q) { *p += 877; return *q * 2; }
static int f5(int *p, int *q) { *p += 708; return *q * 2; }
static int f6(int *p, int *q) { *p += 449; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[1] = cells[1] + 683;
    sum += f1(&cells[6], &cells[1]);
    sum += f2(&cells[3], &cells[3]);
    cells[5] = cells[4] + 311;
    cells[1] = cells[7] + 805;
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 140u; }
    arr[0 % 6] = 940; cells[6] += arr[0 % 6];
    cells[2] = cells[3] + 54;
    arr[5 % 6] = 198; cells[5] += arr[5 % 6];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[7] = cells[4] + 792;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 407u; }
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    sum += f3(&cells[2], &cells[7]);
    sum += f4(&cells[7], &cells[2]);
    arr[1 % 6] = 722; cells[7] += arr[1 % 6];
    sum += f5(&cells[6], &cells[6]);
    sum += f6(&cells[7], &cells[5]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 821u; }
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 58u; }
    arr[4 % 6] = 254; cells[1] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }