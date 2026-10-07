static int f1(int *p, int *q) { *p += 942; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += f1(&cells[3], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    cells[2] = cells[0] + 167;
    arr[5 % 6] = 77; cells[0] += arr[5 % 6];
    cells[5] = cells[5] + 414;
    arr[1 % 6] = 767; cells[2] += arr[1 % 6];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    arr[0 % 6] = 558; cells[5] += arr[0 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    cells[3] = cells[6] + 212;
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    arr[2 % 6] = 484; cells[3] += arr[2 % 6];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 924u; }
    cells[6] = cells[4] + 364;
    arr[5 % 6] = 262; cells[2] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }