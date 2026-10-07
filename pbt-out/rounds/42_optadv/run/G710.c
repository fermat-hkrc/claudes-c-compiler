static int f1(int *p, int *q) { *p += 307; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    arr[2 % 6] = 333; cells[2] += arr[2 % 6];
    cells[3] = cells[2] + 544;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 197u; }
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    arr[6 % 6] = 958; cells[1] += arr[6 % 6];
    arr[3 % 6] = 869; cells[2] += arr[3 % 6];
    arr[1 % 6] = 272; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[6] = cells[5] + 507;
    arr[5 % 6] = 693; cells[3] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[0] = cells[2] + 296;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 503u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    cells[4] = cells[1] + 223;
    cells[7] = cells[0] + 355;
    sum += f1(&cells[5], &cells[5]);
    cells[1] = cells[1] + 224;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 49u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }