static int f1(int *p, int *q) { *p += 871; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[3]);
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    arr[3 % 6] = 124; cells[6] += arr[3 % 6];
    arr[2 % 6] = 848; cells[2] += arr[2 % 6];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 148u; }
    arr[3 % 6] = 681; cells[2] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    cells[1] = cells[1] + 967;
    cells[2] = cells[6] + 712;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    arr[7 % 6] = 192; cells[0] += arr[7 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 687u; }
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 615u; }
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    arr[1 % 6] = 151; cells[4] += arr[1 % 6];
    cells[5] = cells[5] + 649;
    arr[3 % 6] = 833; cells[1] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }