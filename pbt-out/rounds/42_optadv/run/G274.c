static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 884u; }
    cells[5] = cells[5] + 839;
    arr[5 % 6] = 590; cells[7] += arr[5 % 6];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 18u; }
    cells[0] = cells[1] + 889;
    arr[1 % 6] = 617; cells[4] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    arr[1 % 6] = 473; cells[4] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }