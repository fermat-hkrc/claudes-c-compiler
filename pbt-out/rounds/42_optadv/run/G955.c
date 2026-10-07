static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    cells[7] = cells[3] + 214;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 627u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 419u; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    cells[4] = cells[5] + 538;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 127u; }
    cells[4] = cells[7] + 823;
    cells[4] = cells[7] + 558;
    arr[2 % 6] = 590; cells[1] += arr[2 % 6];
    cells[5] = cells[6] + 533;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    arr[1 % 6] = 566; cells[2] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 178u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }