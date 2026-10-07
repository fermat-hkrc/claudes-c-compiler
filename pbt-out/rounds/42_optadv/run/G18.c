static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 988; cells[4] += arr[4 % 6];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[6 % 6] = 698; cells[6] += arr[6 % 6];
    arr[3 % 6] = 648; cells[0] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 986u; }
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    cells[2] = cells[7] + 945;
    arr[4 % 6] = 603; cells[7] += arr[4 % 6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 120u; }
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    arr[4 % 6] = 445; cells[2] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }