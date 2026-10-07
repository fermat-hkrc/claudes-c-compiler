static int f1(int *p, int *q) { *p += 336; return *q * 2; }
static int f2(int *p, int *q) { *p += 211; return *q * 2; }
static int f3(int *p, int *q) { *p += 795; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 675u; }
    arr[4 % 6] = 769; cells[5] += arr[4 % 6];
    cells[1] = cells[1] + 792;
    arr[0 % 6] = 724; cells[7] += arr[0 % 6];
    cells[6] = cells[5] + 791;
    sum += f1(&cells[2], &cells[1]);
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    sum += f2(&cells[4], &cells[4]);
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 792u; }
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 333u; }
    cells[6] = cells[3] + 660;
    arr[3 % 6] = 456; cells[7] += arr[3 % 6];
    cells[3] = cells[2] + 553;
    cells[0] = cells[7] + 637;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f3(&cells[0], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }