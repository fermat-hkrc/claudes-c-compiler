static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 15u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    cells[3] = cells[7] + 800;
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    cells[7] = cells[4] + 687;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 578u; }
    arr[6 % 6] = 140; cells[0] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[7 % 6] = 797; cells[6] += arr[7 % 6];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 204u; }
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    arr[3 % 6] = 146; cells[1] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    cells[5] = cells[4] + 687;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    cells[0] = cells[0] + 307;
    arr[6 % 6] = 940; cells[0] += arr[6 % 6];
    arr[7 % 6] = 542; cells[0] += arr[7 % 6];
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 650u; }
    arr[1 % 6] = 939; cells[2] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }