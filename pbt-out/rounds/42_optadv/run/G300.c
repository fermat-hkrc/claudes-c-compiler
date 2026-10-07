static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 363; cells[3] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 370u; }
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[5 % 6] = 886; cells[1] += arr[5 % 6];
    arr[1 % 6] = 279; cells[6] += arr[1 % 6];
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 741u; }
    cells[6] = cells[5] + 225;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    cells[0] = cells[6] + 642;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 861u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }