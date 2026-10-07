static int f1(int *p, int *q) { *p += 519; return *q * 2; }
static int f2(int *p, int *q) { *p += 922; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    arr[3 % 6] = 829; cells[7] += arr[3 % 6];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[2 % 6] = 768; cells[1] += arr[2 % 6];
    cells[3] = cells[5] + 669;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 429u; }
    cells[6] = cells[3] + 351;
    arr[4 % 6] = 948; cells[3] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 185u; }
    sum += f1(&cells[7], &cells[7]);
    arr[6 % 6] = 221; cells[0] += arr[6 % 6];
    sum += f2(&cells[1], &cells[0]);
    arr[1 % 6] = 494; cells[6] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }