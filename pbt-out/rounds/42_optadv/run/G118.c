static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[0] + 269;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 117u; }
    arr[0 % 6] = 794; cells[4] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[3 % 6] = 549; cells[5] += arr[3 % 6];
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[4] = cells[0] + 749;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[7 % 6] = 174; cells[1] += arr[7 % 6];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    arr[4 % 6] = 238; cells[6] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    arr[2 % 6] = 60; cells[3] += arr[2 % 6];
    arr[6 % 6] = 405; cells[4] += arr[6 % 6];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 658u; }
    cells[5] = cells[0] + 559;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    arr[6 % 6] = 647; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }