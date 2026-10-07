static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[1] = cells[0] + 758;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    cells[7] = cells[0] + 416;
    cells[4] = cells[0] + 658;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 928u; }
    cells[6] = cells[6] + 833;
    arr[6 % 6] = 443; cells[1] += arr[6 % 6];
    cells[3] = cells[3] + 536;
    arr[3 % 6] = 505; cells[3] += arr[3 % 6];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    cells[1] = cells[3] + 730;
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 309u; }
    cells[4] = cells[6] + 204;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }