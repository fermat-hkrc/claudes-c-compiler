static int f1(int *p, int *q) { *p += 8; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    arr[1 % 6] = 7; cells[5] += arr[1 % 6];
    cells[2] = cells[2] + 460;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 500u; }
    arr[0 % 6] = 17; cells[7] += arr[0 % 6];
    arr[7 % 6] = 758; cells[5] += arr[7 % 6];
    arr[4 % 6] = 401; cells[3] += arr[4 % 6];
    sum += f1(&cells[0], &cells[3]);
    cells[2] = cells[7] + 584;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[4 % 6] = 677; cells[4] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    cells[4] = cells[5] + 376;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 422; cells[2] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    cells[3] = cells[0] + 506;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }