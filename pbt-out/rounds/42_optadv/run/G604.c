static int f1(int *p, int *q) { *p += 284; return *q * 2; }
static int f2(int *p, int *q) { *p += 717; return *q * 2; }
static int f3(int *p, int *q) { *p += 477; return *q * 2; }
static int f4(int *p, int *q) { *p += 543; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 96u; }
    cells[7] = cells[7] + 727;
    sum += f1(&cells[0], &cells[5]);
    cells[0] = cells[4] + 796;
    cells[0] = cells[5] + 942;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 752u; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 280u; }
    sum += f2(&cells[7], &cells[2]);
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += f3(&cells[4], &cells[1]);
    cells[6] = cells[6] + 470;
    sum += f4(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[0] = cells[5] + 508;
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[6] = cells[1] + 838;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }