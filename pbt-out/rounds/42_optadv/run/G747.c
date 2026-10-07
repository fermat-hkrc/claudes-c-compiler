static int f1(int *p, int *q) { *p += 315; return *q * 2; }
static int f2(int *p, int *q) { *p += 711; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 270; cells[4] += arr[3 % 6];
    arr[3 % 6] = 596; cells[5] += arr[3 % 6];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 819u; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    arr[3 % 6] = 62; cells[3] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[3], &cells[6]);
    cells[3] = cells[3] + 326;
    arr[3 % 6] = 904; cells[3] += arr[3 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 630u; }
    cells[1] = cells[0] + 816;
    cells[7] = cells[0] + 371;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 24u; }
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 632u; }
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 995u; }
    sum += f2(&cells[4], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }