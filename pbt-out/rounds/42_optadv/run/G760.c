static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 487; cells[4] += arr[4 % 6];
    arr[2 % 6] = 473; cells[6] += arr[2 % 6];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 579u; }
    cells[7] = cells[7] + 274;
    arr[5 % 6] = 691; cells[4] += arr[5 % 6];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 265u; }
    cells[4] = cells[2] + 429;
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[5 % 6] = 672; cells[6] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    arr[2 % 6] = 240; cells[1] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }