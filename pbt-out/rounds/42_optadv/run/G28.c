static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    arr[2 % 6] = 225; cells[1] += arr[2 % 6];
    arr[7 % 6] = 268; cells[4] += arr[7 % 6];
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 476u; }
    arr[4 % 6] = 183; cells[1] += arr[4 % 6];
    arr[3 % 6] = 840; cells[3] += arr[3 % 6];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    arr[4 % 6] = 839; cells[4] += arr[4 % 6];
    arr[7 % 6] = 86; cells[6] += arr[7 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 245u; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 634u; }
    arr[7 % 6] = 946; cells[2] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 645u; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 390u; }
    cells[7] = cells[1] + 625;
    arr[5 % 6] = 705; cells[3] += arr[5 % 6];
    arr[3 % 6] = 881; cells[1] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }