static int f1(int *p, int *q) { *p += 587; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 529; cells[7] += arr[0 % 6];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[0 % 6] = 74; cells[1] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 981u; }
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 726u; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 985u; }
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 70u; }
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    cells[0] = cells[6] + 73;
    arr[2 % 6] = 14; cells[1] += arr[2 % 6];
    arr[0 % 6] = 891; cells[0] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[7 % 6] = 623; cells[4] += arr[7 % 6];
    sum += f1(&cells[5], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }